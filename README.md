# Grocery Automation with Claude Code

A **no-code, conversational automation system** for grocery shopping at Giant Food Stores using Claude Code with Playwright MCP and Qdrant recipe storage.

## What Is This?

This is a collection of **markdown files and instructions** that enable Claude Code to automate your entire grocery shopping workflow through natural conversation. No traditional code required—just talk to Claude, and it uses browser automation to:

- Search for products on giantfoodstores.com
- Add items to your cart based on your preferences
- Reference your weekly staples and past orders
- Store and retrieve recipes for meal planning
- Validate orders against your usual shopping list

## How It Works

The system uses three key technologies:

1. **Claude Code** - AI assistant that reads your markdown instructions and executes tasks
2. **Playwright MCP** - Browser automation tool that Claude uses to interact with the Giant Food Stores website
3. **Qdrant Knowledge Base** - Vector database for storing and searching recipes

Instead of writing traditional code, you maintain **markdown files** that serve as instructions and reference data. Claude reads these files and automates shopping based on your preferences.

## Setup

### Prerequisites

- [Claude Code](https://claude.com/claude-code) installed
- Node.js (for Playwright MCP)
- Giant Food Stores account

### Installation Steps

#### 1. Install Playwright MCP

```bash
claude mcp add playwright npx '@playwright/mcp@latest'
```

#### 2. Clone This Repository

```bash
git clone https://github.com/DSado88/Grocery.git
cd Grocery
```

#### 3. Configure Environment Variables

Create a `.env` file in the project root:

```env
EMAIL=your-giant-account@email.com
PASSWORD=your-password
```

**⚠️ IMPORTANT:** The `.env` file is gitignored. Never commit credentials to version control.

#### 4. Set Default Store Location

The system defaults to:
```
GIANT
225 Lancaster Ave
Malvern, PA 19355
(610) 296-5551
```

To use a different store, edit the location in `CLAUDE.md`.

#### 5. Review and Customize Reference Files

Edit these markdown files to match your preferences:
- `weekly-staples-list.md` - Items you buy every week
- `preferred-items-reference.md` - Brand and specification preferences
- `periodic-household-items.md` - Household items to check regularly
- `grocery-orders-history.md` - Your past shopping patterns

## Project Structure

```
Grocery/
├── CLAUDE.md                          # Main automation instructions
├── weekly-staples-list.md             # Always-buy items
├── preferred-items-reference.md       # Brand preferences
├── grocery-orders-history.md          # Order history
├── last-order-items.md                # Most recent order
├── periodic-household-items.md        # Household staples
├── recipe-links.json                  # Saved recipe URLs
├── COLLECTION_INFO.md                 # Qdrant collection info
├── start-grocery-session.sh           # Quick start script
└── .env                               # Credentials (gitignored)
```

### Key Files

#### `CLAUDE.md`
The **master instruction file** containing:
- Website structure and navigation patterns
- Playwright automation selectors and scripts
- Shopping list validation rules
- Recipe storage workflows
- Pantry staples intelligence

#### Reference Data Files
Markdown files that serve as Claude's "memory":
- **weekly-staples-list.md** - Core items for every order
- **preferred-items-reference.md** - "Get THIS brand, not that one"
- **periodic-household-items.md** - Paper products, cleaning supplies, etc.
- **grocery-orders-history.md** - Pattern recognition for suggestions

## Usage

### Starting a Grocery Session

#### Option 1: Quick Start Script (Recommended)

Use the included shell script for a guided workflow:

```bash
./start-grocery-session.sh
```

This script:
1. Launches Claude Code
2. Instructs it to login to Giant Food Stores
3. Reads your weekly staples
4. Asks which items to skip
5. Adds items to cart
6. Prompts for household items

#### Option 2: Manual Conversation

Simply open Claude Code in this directory and start talking:

```bash
cd Grocery
claude
```

Then say: `"I need my weekly staples"` or `"Let's build a grocery order"`

### Creating the Start Script

If you need to recreate or customize `start-grocery-session.sh`:

```bash
#!/bin/bash

# Navigate to project directory
cd /Users/YOUR_USERNAME/Documents/Programs/Grocery

# Launch Claude with instructions
claude "Start my grocery shopping session. Follow these steps EXACTLY:

IMPORTANT SETUP:
- ALWAYS reference CLAUDE.md for store location, automation instructions, and pantry intelligence
- Use the Playwright MCP browser tools for all Giant Food Stores interactions
- Check and use login credentials from the .env file
- MUST login to Giant Food Stores BEFORE attempting to add any items to cart
- Navigate to https://giantfoodstores.com/ and complete login first

STEP 1: LOGIN
- Use Playwright to navigate to Giant Food Stores
- Login using credentials from .env file
- Verify login was successful before proceeding

STEP 2: WEEKLY STAPLES
- Read and list ALL items from weekly-staples-list.md
- Ask me: 'Here are your weekly staples. Which items should I SKIP this week? (Just list the items you DON'T want, or say none)'
- After I respond, add ALL non-skipped items to the Giant cart using Playwright
- IMPORTANT: Don't stop if one item has an issue - continue with the next item until all are attempted
- Report which items were successfully added and which had issues

STEP 3: HOUSEHOLD ITEMS
- After completing all staples, read and list items from periodic-household-items.md
- Ask me: 'Which household items would you like to add this week?'
- Add only the items I specify using Playwright

Remember: Always use CLAUDE.md as your reference guide and Playwright MCP for browser automation.

Start now by logging into Giant Food Stores."
```

Make it executable:
```bash
chmod +x start-grocery-session.sh
```

### The Antiprompt: What NOT to Do

**❌ DON'T ask Claude to:**
- "Write a Python script to scrape the website" - No code needed!
- "Install Selenium" - Use Playwright MCP instead
- "Create a database" - Use markdown files and Qdrant
- "Build a web interface" - The interface is conversation
- "Write API calls" - Playwright handles browser interaction
- "Set up a backend server" - Everything runs locally
- "Create a config.json" - Instructions live in markdown

**✅ DO ask Claude to:**
- "Add my weekly staples"
- "Search for Nature's Promise ground chicken"
- "Save this recipe URL"
- "What household items do I need to check?"
- "Show me recipes with tofu"
- "I want to make bulgogi burgers this week"
- "Skip the bananas this week"

### What You DON'T Need

This is a **no-code system**. You don't need:

- ❌ Programming knowledge (Python, JavaScript, etc.)
- ❌ Separate automation tools (Selenium, Puppeteer, etc.)
- ❌ Web scraping libraries (BeautifulSoup, Scrapy, etc.)
- ❌ API integration or API keys
- ❌ Database setup (beyond Qdrant MCP)
- ❌ Web server or hosting
- ❌ Complex JSON/YAML configuration files
- ❌ Docker containers or virtual environments
- ❌ Cron jobs for scheduling

**All you need:**
- ✅ Claude Code (the AI assistant)
- ✅ Playwright MCP (installed via one command)
- ✅ Markdown files (your "database")
- ✅ Natural conversation
- ✅ A Giant Food Stores account

### Typical Workflow Prompts

**Starting a standard weekly order:**
```
You: ./start-grocery-session.sh
Claude: *Logs in, reads weekly-staples-list.md*
Claude: "Here are your weekly staples: [lists items]. Which items should I SKIP this week?"
You: "Skip the strawberries and limes"
Claude: *Adds all items except strawberries and limes*
Claude: "Successfully added 18 items. 2 items skipped. Which household items would you like?"
You: "Add paper towels and toilet paper"
Claude: *Searches and adds Bounty paper towels and Charmin toilet paper*
```

**Recipe-based shopping:**
```
You: "Save this recipe: https://pinchofyum.com/bulgogi-burgers"
Claude: *Fetches recipe, stores in food_recipes collection*
Claude: "Recipe saved! Ingredients: ground beef, gochujang, soy sauce, garlic..."

You: "I want to make bulgogi burgers this week"
Claude: *Retrieves recipe from Qdrant*
Claude: "I'll add ground beef, lettuce, pickles, and brioche buns. The recipe uses gochujang and soy sauce—do you have these pantry staples?"
You: "Need gochujang"
Claude: "Gochujang isn't available at Giant. You'll need to get it from H Mart or Amazon. Adding the other ingredients now..."
```

**Mid-week additions:**
```
You: "Add 2 gallons of milk and eggs"
Claude: *Searches for preferred brands, adds to cart*
Claude: "Added GIANT Whole Milk (2 gallons) and GIANT Grade A Large Eggs (1 dozen)"
```

### Example Conversations

**Weekly staples order:**
```
You: "I need my weekly staples"
Claude: *Navigates to Giant Food Stores, searches for and adds all items from weekly-staples-list.md*
```

**Recipe-based shopping:**
```
You: "I want to make bulgogi burgers and sambal noodles this week"
Claude: *Searches food_recipes collection, extracts ingredients, adds to cart*
Claude: "The recipes need soy sauce and gochujang - do you have these pantry staples?"
```

**Custom order:**
```
You: "Add 2 gallons of milk, a dozen eggs, and Nature's Promise ground chicken"
Claude: *Searches for items using preferred brands, adds to cart*
```

**Big shopping trip:**
```
You: "I need a big shop this week"
Claude: *Adds weekly staples, then asks about periodic items*
Claude: "How are you on paper towels, toilet paper, and cleaning supplies?"
```

## Recipe Management with Qdrant

The system uses a Qdrant collection called `food_recipes` to store recipes.

### Adding Recipes

**From a URL:**
```
You: "Save this recipe: https://pinchofyum.com/bulgogi-burgers"
Claude: *Fetches recipe, extracts ingredients, stores in food_recipes collection*
Claude: "Recipe saved! Note: Gochugaru not available at Giant - you'll need H Mart"
```

**Searching Recipes:**
```
You: "What recipes do I have with tofu?"
Claude: *Searches food_recipes collection, lists matching recipes*
```

**Cooking from Saved Recipes:**
```
You: "I want to make those bulgogi burgers again"
Claude: *Retrieves recipe from collection, generates shopping list*
Claude: "I'll add: ground beef, lettuce, pickles, brioche buns..."
```

### Recipe Storage Format

Recipes are stored with:
- URL (source)
- Full ingredients list with quantities
- Shopping notes (availability at Giant)
- Cuisine type
- Main protein
- Cook time
- Any substitutions

## Automation Features

### Intelligent Shopping Assistant

**Staples Validation:**
After building your cart, Claude automatically checks:
- Are all weekly staples included?
- Correct quantities for frequently-bought items?
- Any missing items from your usual pattern?

**Pantry Intelligence:**
Claude knows which items are long-lasting and asks before adding:
- Condiments (mayo, ketchup, soy sauce, etc.)
- Baking supplies (flour, sugar, baking powder)
- Oils and vinegars
- Spices and seasonings

**Pattern Recognition:**
- Tracks your order frequency
- Suggests items based on past purchases
- Prompts for periodic household items every 3-4 orders

### Shopping List Generation

From recipes, Claude:
1. Extracts all ingredients
2. Cross-references with your recent orders
3. Checks pantry staples
4. Consolidates quantities across multiple recipes
5. Notes items not available at Giant

## Playwright Automation Details

The system uses Playwright MCP to:
- Navigate giantfoodstores.com
- Search for products by name
- Select preferred brands based on your specs
- Add items to cart with correct quantities
- Handle store location selection
- Manage cart updates

**All automation is handled conversationally** - you never write Playwright code yourself.

## How the Markdown "Database" Works

Instead of traditional databases, this system uses **structured markdown files** that Claude reads and interprets:

### `weekly-staples-list.md`
```markdown
## Produce
- Cucumber (1)
- Cilantro (1 bunch)
- Limes (1-2)

## Dairy
- Icelandic Provisions Vanilla Skyr (3-5 cups)
```

### `preferred-items-reference.md`
```markdown
### Ground Chicken
- **Brand:** Nature's Promise 99% Lean Ground Chicken
- **Size:** ~1 lb packages
- **Notes:** White meat only, no dark meat
```

Claude reads these files during conversation and uses them to guide automation decisions.

## Security & Privacy

- **Credentials:** Stored in `.env` (gitignored, never committed)
- **Private Data:** All reference files stay local
- **Browser Automation:** Runs locally on your machine
- **Recipe Data:** Stored in local Qdrant collection
- **No Cloud Services:** Everything runs on your computer

## Customization

### Changing Default Store

Edit the store location in `CLAUDE.md`:
```markdown
## Default Store Location
**ALWAYS USE THIS STORE UNLESS OTHERWISE SPECIFIED:**
```
GIANT
[Your Store Address]
[Your ZIP Code]
```
```

### Adding New Staples

Edit `weekly-staples-list.md`:
```markdown
- New Item Name (Brand, Size)
```

### Setting Brand Preferences

Edit `preferred-items-reference.md`:
```markdown
### Category
- **Item:** Preferred Brand Name, Specific Details
```

### Customizing the Start Script

Edit `start-grocery-session.sh` to change the workflow:
- Add/remove steps
- Change question prompts
- Adjust the order of operations
- Add recipe suggestions

## Tips & Best Practices

1. **Be Specific:** "Nature's Promise ground chicken" works better than just "chicken"
2. **Review Before Checkout:** Claude adds to cart but doesn't complete checkout automatically
3. **Update References:** Keep markdown files current with your changing preferences
4. **Save Recipes:** Build up your recipe collection for faster meal planning
5. **Ask Questions:** Claude can explain what it's doing or why it chose specific items
6. **Use the Script:** `start-grocery-session.sh` provides a consistent workflow
7. **Skip Items:** It's easier to skip items when prompted than to remove them later
8. **Pantry Check:** Let Claude ask about pantry staples—saves time and money

## Troubleshooting

**"Store location not found"**
- Update ZIP code in `CLAUDE.md`
- Verify store address on giantfoodstores.com

**"Item not found"**
- Check brand name in `preferred-items-reference.md`
- Try more generic search terms
- Ask Claude to search for alternatives

**"Authentication failed"**
- Verify `.env` credentials are correct
- Check if Giant website layout changed
- Try logging in manually first

**Playwright MCP not working:**
```bash
# Reinstall Playwright MCP
claude mcp add playwright npx '@playwright/mcp@latest'
```

**Script won't run:**
```bash
# Make executable
chmod +x start-grocery-session.sh

# Check path in script matches your directory
```

**Claude isn't following CLAUDE.md:**
- Mention `CLAUDE.md` explicitly: "Check CLAUDE.md for the default store"
- Restart Claude Code session
- Verify `CLAUDE.md` is in the project root

## Advanced: Building Your Own Version

Want to adapt this for a different grocery store? Here's what to change:

1. **Replace the website:** Update URLs in `CLAUDE.md` and `start-grocery-session.sh`
2. **Map the selectors:** Use Playwright's `browser_snapshot` to identify buttons, search boxes, etc.
3. **Document the flow:** Write down the navigation steps in `CLAUDE.md`
4. **Update store location:** Change default store in `CLAUDE.md`
5. **Customize reference files:** Adapt to your shopping patterns

The beauty of this system: **no code changes needed**, just update the markdown instructions!

## Why This Approach?

**Traditional automation:**
- Write Python/JavaScript code
- Maintain complex dependencies
- Debug brittle selectors
- Update code when website changes

**This system:**
- Write natural language instructions
- Claude handles the automation
- Describe what you want, not how to do it
- Update instructions when needed

**Result:** More maintainable, accessible to non-programmers, easier to customize.

## Contributing

This is a personal grocery automation system, but feel free to fork and adapt for:
- Other grocery stores (update `CLAUDE.md` selectors)
- Different shopping patterns (modify reference files)
- Additional features (extend instruction files)
- Other countries/languages

## License

MIT

## Acknowledgments

Built with:
- [Claude Code](https://claude.com/claude-code) by Anthropic
- [Playwright](https://playwright.dev/) for browser automation
- [Qdrant](https://qdrant.tech/) for recipe storage
- Giant Food Stores for... existing

---

**No code. Just conversation. Automated groceries.** 🛒🤖
