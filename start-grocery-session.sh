#!/bin/bash

# Grocery Shopping Assistant - Auto-start Script
# This script launches Claude Code with specific instructions for the grocery shopping workflow

# Navigate to the Grocery directory
cd /Users/david/Documents/Programs/Grocery

# Launch Claude Code with detailed instructions for the shopping session
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