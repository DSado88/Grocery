use super::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// ── Defect 1: ReadlineError Io should be recoverable, not crash the REPL ────

#[test]
fn test_is_recoverable_readline_error_io_returns_true() -> TestResult {
    let err = rustyline::error::ReadlineError::Io(std::io::Error::new(
        std::io::ErrorKind::Interrupted,
        "signal",
    ));
    assert!(
        is_recoverable_readline_error(&err),
        "Io errors (e.g. EINTR from terminal resize) should be recoverable"
    );
    Ok(())
}

#[test]
fn test_is_recoverable_readline_error_eof_returns_false() -> TestResult {
    assert!(
        !is_recoverable_readline_error(&rustyline::error::ReadlineError::Eof),
        "Eof should NOT be recoverable — it means the input stream ended"
    );
    Ok(())
}

#[test]
fn test_is_recoverable_readline_error_interrupted_returns_false() -> TestResult {
    assert!(
        !is_recoverable_readline_error(&rustyline::error::ReadlineError::Interrupted),
        "Interrupted (Ctrl+C) should NOT be recoverable — handled as exit"
    );
    Ok(())
}

// ── Defect 2: SpawnFailed gives generic error, should include install guidance ──

#[test]
fn test_format_user_facing_error_spawn_failed_contains_guidance() -> TestResult {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "No such file or directory");
    let butler_err = butler_claude::Error::SpawnFailed(io_err);
    let chat_err = ChatError::Claude(butler_err);
    let msg = format_user_facing_error(&chat_err);

    assert!(
        msg.to_lowercase().contains("install")
            || msg.to_lowercase().contains("not found")
            || msg.to_lowercase().contains("path"),
        "SpawnFailed should give install guidance, got: {msg}"
    );
    Ok(())
}

#[test]
fn test_format_user_facing_error_context_error_passes_through() -> TestResult {
    let err = ChatError::Context("household-model.yaml: file not found".to_string());
    let msg = format_user_facing_error(&err);

    assert!(
        msg.contains("household-model.yaml"),
        "Non-spawn errors should pass through unchanged, got: {msg}"
    );
    Ok(())
}

// ── Defect 3: Session death mid-response doesn't tell user to resend ──────────

#[test]
fn test_session_death_notice_mentions_resend() -> TestResult {
    assert!(
        SESSION_DEATH_NOTICE.to_lowercase().contains("resend")
            || SESSION_DEATH_NOTICE.to_lowercase().contains("re-send")
            || SESSION_DEATH_NOTICE.to_lowercase().contains("repeat")
            || SESSION_DEATH_NOTICE.to_lowercase().contains("again"),
        "Session death notice should tell user to resend their message, got: {SESSION_DEATH_NOTICE}"
    );
    Ok(())
}

// ── Defect 4: read_line JoinError handler contains expect() panic ─────────────

#[test]
fn test_read_line_no_expect_or_panic_in_source() -> TestResult {
    // Structural test: verify the source code doesn't contain expect() calls.
    // This catches the no-panic policy violation at compile-test time.
    let source = include_str!("chat.rs");

    // Filter out test code and comments — only check production code
    let prod_lines: Vec<&str> = source
        .lines()
        .take_while(|line| !line.contains("#[cfg(test)]"))
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.starts_with("//") && !trimmed.starts_with("///")
        })
        .collect();
    let prod_code = prod_lines.join("\n");

    assert!(
        !prod_code.contains(".expect("),
        "Production code must not contain .expect() — violates no-panic policy"
    );
    assert!(
        !prod_code.contains(".unwrap()"),
        "Production code must not contain .unwrap() — violates no-panic policy"
    );
    Ok(())
}
