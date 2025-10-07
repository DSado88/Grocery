# Claude Code Knowledge Base Collection

This project uses the Qdrant collection: **food_recipes**

## Using the Knowledge Base

When using Claude Code in this project, you can:

1. **Store information**: Ask Claude to store code patterns, documentation, or project knowledge
   - Example: "Store this authentication flow pattern for future reference"
   - The data will be saved to the 'food_recipes' collection

2. **Search information**: Ask Claude to find previously stored information
   - Example: "Find all stored information about API endpoints"
   - Claude will search the 'food_recipes' collection

## Global MCP Server

This project uses the globally installed 'claude-knowledge' MCP server.
The collection name is stored in .claude-collection file.

## Management

To manage this collection, use:
```bash
cd /Users/david/Documents/Programs/claude-code-knowledge-base
./manage-collections.sh
```

Then select option 2 to view information about 'food_recipes'
