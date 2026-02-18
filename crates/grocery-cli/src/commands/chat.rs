use std::io::Write;
use std::path::Path;

use butler_claude::{ClaudeMessage, Session, SessionOptions};
use grocery_core::household::HouseholdModel;
use grocery_core::types::FrequencyTier;
use grocery_recipes::RecipeCollection;
use thiserror::Error;
use tracing::{info, warn};

#[derive(Error, Debug)]
pub enum ChatError {
    #[error("household context error: {0}")]
    Context(String),

    #[error("claude session error: {0}")]
    Claude(#[from] butler_claude::Error),

    #[error("readline error: {0}")]
    Readline(#[from] rustyline::error::ReadlineError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Classify readline errors as recoverable (skip and continue) or fatal (exit REPL).
///
/// Io errors (e.g. EINTR from SIGWINCH on terminal resize) are transient —
/// the REPL should print a warning and continue, not crash.
fn is_recoverable_readline_error(err: &rustyline::error::ReadlineError) -> bool {
    matches!(err, rustyline::error::ReadlineError::Io(_))
}

/// Format a ChatError with actionable user guidance.
///
/// SpawnFailed gets special treatment: users need to know how to install Claude CLI.
/// All other errors pass through unchanged.
pub fn format_user_facing_error(err: &ChatError) -> String {
    match err {
        ChatError::Claude(butler_claude::Error::SpawnFailed(io_err)) => {
            format!(
                "Failed to start Claude CLI: {io_err}\n\
                 Is 'claude' installed and in your PATH?\n\
                 Install from: https://claude.ai/download"
            )
        }
        other => other.to_string(),
    }
}

/// Message shown when session dies mid-response and conversation context is lost.
const SESSION_DEATH_NOTICE: &str =
    "[Session died — respawning fresh session. Please resend your last message.]";

/// Build a concise household context string for append_system_prompt.
fn build_household_context(data_dir: &Path) -> Result<String, ChatError> {
    let household = HouseholdModel::from_file(&data_dir.join("household-model.yaml"))
        .map_err(|e| ChatError::Context(format!("household-model.yaml: {e}")))?;

    let collection = RecipeCollection::from_json_file(&data_dir.join("recipe-links.json"))
        .map_err(|e| ChatError::Context(format!("recipe-links.json: {e}")))?;

    let mut ctx = String::with_capacity(1024);

    // Family
    ctx.push_str("## Household Context\n\nFamily: ");
    let members: Vec<String> = household
        .family
        .members
        .iter()
        .map(|m| {
            m.age
                .map(|a| format!("{} (age {})", m.name, a))
                .unwrap_or_else(|| m.name.clone())
        })
        .collect();
    ctx.push_str(&members.join(", "));
    ctx.push('\n');

    // Giant items summary
    let tier1 = household.giant_items_by_tier(FrequencyTier::EveryOrder);
    ctx.push_str(&format!(
        "\nGiant: {} recurring items ({} staples/every-order)\n",
        household.giant_recurring.len(),
        tier1.len()
    ));

    // Amazon summary
    if !household.amazon_recurring.is_empty() {
        ctx.push_str(&format!(
            "Amazon: {} recurring items\n",
            household.amazon_recurring.len()
        ));
    }

    // Recipe collection
    let total = collection.len();
    let with_ingredients = collection.with_ingredients().len();
    ctx.push_str(&format!(
        "\nRecipes: {} total ({} with ingredients)\n",
        total, with_ingredients
    ));

    // Top proteins
    let mut protein_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for recipe in collection.recipes() {
        if let Some(ref p) = recipe.primary_protein {
            *protein_counts.entry(p.clone()).or_default() += 1;
        }
    }
    let mut proteins: Vec<_> = protein_counts.into_iter().collect();
    proteins.sort_by(|a, b| b.1.cmp(&a.1));

    if !proteins.is_empty() {
        ctx.push_str("Top proteins: ");
        let top: Vec<String> = proteins
            .iter()
            .take(5)
            .map(|(p, c)| format!("{p} ({c})"))
            .collect();
        ctx.push_str(&top.join(", "));
        ctx.push('\n');
    }

    // Data directory for file access
    ctx.push_str(&format!("\nData directory: {}\n", data_dir.display()));
    ctx.push_str("Key files: household-model.yaml, recipe-links.json, recipe-scoring-config.yaml\n");

    Ok(ctx)
}

/// Build SessionOptions for the Claude session.
fn build_session_options(context: &str, model: Option<String>) -> SessionOptions {
    SessionOptions {
        append_system_prompt: Some(context.to_string()),
        allowed_tools: vec![
            "Read".to_string(),
            "Grep".to_string(),
            "Glob".to_string(),
        ],
        permission_mode: "bypassPermissions".to_string(),
        max_turns: Some(25),
        model,
        ..Default::default()
    }
}

/// Stream Claude's response, printing text blocks and result metadata.
/// Returns true if any text was printed.
async fn stream_response(session: &mut Session) -> Result<bool, butler_claude::Error> {
    let mut had_text = false;
    let mut had_content_block = false;

    while let Some(msg) = session.next_message().await? {
        if let ClaudeMessage::Assistant(ref assistant) = msg {
            // Prefer content_block for streaming display
            if let Some(ref block) = assistant.content_block {
                if let butler_claude::ContentBlock::Text { ref text } = block {
                    print!("{text}");
                    let _ = std::io::stdout().flush();
                    had_text = true;
                    had_content_block = true;
                }
            }

            // Fall back to full message text if no content_block was streamed
            if !had_content_block {
                if let Some(text) = msg.text_content() {
                    if !text.is_empty() {
                        print!("{text}");
                        let _ = std::io::stdout().flush();
                        had_text = true;
                    }
                }
            }
        }

        if msg.is_final() {
            if had_text {
                println!();
            }
            // Print cost/duration metadata to stderr
            if let ClaudeMessage::Result(ref result) = msg {
                let cost = result.cost_usd.unwrap_or(0.0);
                let duration = result.duration_ms.unwrap_or(0);
                let turns = result.num_turns.unwrap_or(0);
                eprintln!(
                    "[${:.4} | {:.1}s | {} turns]",
                    cost,
                    duration as f64 / 1000.0,
                    turns
                );
            }
            break;
        }
    }

    Ok(had_text)
}

/// Send a message and stream the response, with session death recovery.
/// Returns the (possibly respawned) session.
async fn send_and_stream(
    mut session: Session,
    message: &str,
    options: &SessionOptions,
) -> Result<Session, ChatError> {
    // Inject user message
    if let Err(e) = session.inject_user_message(message).await {
        if matches!(e, butler_claude::Error::SessionDied) {
            warn!("Session died on inject — respawning");
            eprintln!("{SESSION_DEATH_NOTICE}");
            session = Session::warmup(options.clone()).await?;
            session.inject_user_message(message).await?;
        } else {
            return Err(ChatError::Claude(e));
        }
    }

    // Stream response
    match stream_response(&mut session).await {
        Ok(_) => {}
        Err(butler_claude::Error::SessionDied) => {
            warn!("Session died mid-response — respawning");
            eprintln!("\n{SESSION_DEATH_NOTICE}");
            session = Session::warmup(options.clone()).await?;
        }
        Err(e) => return Err(ChatError::Claude(e)),
    }

    Ok(session)
}

/// Read a line from the user using rustyline on a blocking thread.
///
/// Takes ownership of the editor (move into blocking task), returns it in the tuple
/// to preserve readline history state across calls. Returns `None` if the editor
/// was lost (e.g. spawn_blocking task panicked).
enum ReadlineResult {
    Line(String),
    Exit,
    Error(rustyline::error::ReadlineError),
}

async fn read_line(
    rl: rustyline::DefaultEditor,
) -> Option<(rustyline::DefaultEditor, ReadlineResult)> {
    let result = tokio::task::spawn_blocking(move || {
        let mut rl = rl;
        let line = rl.readline("you> ");
        (rl, line)
    })
    .await;

    match result {
        Ok((rl, Ok(line))) => Some((rl, ReadlineResult::Line(line))),
        Ok((rl, Err(rustyline::error::ReadlineError::Interrupted)))
        | Ok((rl, Err(rustyline::error::ReadlineError::Eof))) => Some((rl, ReadlineResult::Exit)),
        Ok((rl, Err(e))) => Some((rl, ReadlineResult::Error(e))),
        // JoinError: the blocking task panicked — editor is lost, signal exit
        Err(join_err) => {
            warn!("readline task failed: {join_err}");
            None
        }
    }
}

/// Run a single-turn chat: send message, print response, exit.
async fn run_single_turn(prompt: &str, options: SessionOptions) -> Result<(), ChatError> {
    info!(prompt_len = prompt.len(), "Single-turn chat");
    let mut session = Session::new(prompt, options).await?;
    stream_response(&mut session).await?;
    Ok(())
}

/// Run the interactive REPL loop with an existing session.
async fn repl_loop(
    mut session: Session,
    options: &SessionOptions,
) -> Result<(), ChatError> {
    let mut rl = rustyline::DefaultEditor::new()?;

    loop {
        let Some((returned_rl, result)) = read_line(rl).await else {
            // Editor lost (JoinError) — exit cleanly
            eprintln!("Input handler failed — exiting.");
            break;
        };
        rl = returned_rl;

        match result {
            ReadlineResult::Exit => {
                eprintln!("Goodbye!");
                break;
            }
            ReadlineResult::Error(e) => {
                if is_recoverable_readline_error(&e) {
                    eprintln!("[Input error: {e} — try again]");
                    continue;
                }
                return Err(ChatError::Readline(e));
            }
            ReadlineResult::Line(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed.eq_ignore_ascii_case("quit")
                    || trimmed.eq_ignore_ascii_case("exit")
                {
                    eprintln!("Goodbye!");
                    break;
                }

                session = send_and_stream(session, trimmed, options).await?;
                println!(); // Blank line before next prompt
            }
        }
    }

    Ok(())
}

/// Entry point for the `chat` subcommand.
pub async fn run(
    data_dir: &Path,
    initial_message: Option<&str>,
    model: Option<String>,
    once: bool,
) -> Result<(), ChatError> {
    let context = build_household_context(data_dir)?;
    info!(context_len = context.len(), "Built household context");

    let options = build_session_options(&context, model);

    if once {
        let prompt = initial_message.unwrap_or("What can you help me with?");
        return run_single_turn(prompt, options).await;
    }

    // REPL mode
    eprintln!("Cart Blanche REPL (type 'quit' to exit)\n");

    let session = match initial_message {
        Some(msg) => {
            let mut session = Session::new(msg, options.clone()).await?;
            eprintln!("you> {msg}");
            stream_response(&mut session).await?;
            println!();
            session
        }
        None => {
            eprintln!("Warming up Claude session...");
            let session = Session::warmup(options.clone()).await?;
            eprintln!("Ready.\n");
            session
        }
    };

    repl_loop(session, &options).await
}

#[cfg(test)]
#[path = "chat_tests.rs"]
mod tests;
