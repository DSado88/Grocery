# Giant Food Stores - Playwright Automation Guide

## Default Store Location
**ALWAYS USE THIS STORE UNLESS OTHERWISE SPECIFIED:**
```
GIANT
225 Lancaster Ave
Malvern, PA 19355
(610) 296-5551
```

## Website Structure Map

### Main Site: https://giantfoodstores.com/

#### 1. Homepage Elements
- **Header Navigation**
  - Menu button (hamburger menu)
  - Home logo/link
  - Search bar (main product search)
  - Cart button with subtotal display

- **Store Selector**
  - Location display (e.g., "700 Nutt Rd, 19460")
  - In-Store/Pickup/Delivery toggle

- **Main Content Areas**
  - Hero carousel with weekly deals
  - Quick links: Coupons, Weekly Ad, GIANT Choice Rewards, Store Locator
  - Product recommendations sections
  - Trending/Featured items

#### 2. Product Search Pages
**URL Pattern:** `/product-search/{search-term}?searchRef=&semanticSearch=false`

- **Search Results Header**
  - Result count (e.g., "739 results for 'milk'")
  - Filter Options button
  - Sort dropdown (Best Match, Most Popular, Price Low/High, Unit Price, Aisle)
  - View toggles (list/grid)

- **Trending Searches Bar**
  - Category-specific quick filters
  - Horizontal scrollable list

- **Product Tiles**
  - Product image
  - Product name
  - Price and unit price
  - "Add to cart" button
  - Quantity controls (after adding)
  - Coupon/deal badges
  - "Add to shopping list" button

#### 3. Product Detail Pages
**URL Pattern:** `/product/{product-name}/{product-id}`

- Product images
- Pricing information
- Quantity selector
- Add to cart functionality
- Product details/nutrition info

#### 4. Shopping Cart
- Running total in header
- Full cart page (accessible via cart button)
- Checkout flow

## Playwright Automation Instructions

### Initial Setup
```javascript
// Navigate to Giant Food Stores
await page.goto('https://giantfoodstores.com/');

// Wait for page load
await page.waitForLoadState('networkidle');
```

### Store Selection
```javascript
// Check current store location
const storeLocation = await page.locator('button:has-text("Lancaster Ave")').textContent();

// If not the correct store, click to change
if (!storeLocation.includes('225 Lancaster Ave')) {
    await page.click('button:has-text("Nutt Rd")'); // Click current store
    // Handle store selection modal
    await page.fill('input[placeholder="Enter ZIP code"]', '19355');
    await page.click('button:has-text("Search")');
    await page.click('text=225 Lancaster Ave');
}
```

### Product Search
```javascript
// Use search bar
await page.fill('searchbox', 'product_name');
await page.press('searchbox', 'Enter');

// Wait for results
await page.waitForSelector('heading:has-text("Search Results")');
```

### Adding Items to Cart
```javascript
// Method 1: Direct add from search results
await page.click('button:has-text("Add to cart"):nth(index)');

// Method 2: With specific product
await page.locator('article:has-text("Product Name")').locator('button:has-text("Add to cart")').click();

// Verify cart update
await page.waitForSelector('button:has-text("Cart Subtotal $")');
```

### Cart Management
```javascript
// View cart total
const cartTotal = await page.locator('button[aria-label*="Cart Subtotal"]').textContent();

// Adjust quantity (after item is in cart)
await page.click('button[aria-label="Increase quantity"]');
await page.click('button[aria-label="Decrease quantity"]');
```

### Common Selectors Reference

#### By Element Type
- Search bar: `searchbox`
- Cart button: `button:has-text("Cart Subtotal")`
- Add to cart buttons: `button:has-text("Add to cart")`
- Product tiles: `article` or `listitem` containing product info

#### By Ref IDs (when available)
- Use refs from browser_snapshot for precise targeting
- Example: `[ref="e1134"]` for specific add to cart button

### Best Practices

1. **Always wait for page loads**
   ```javascript
   await page.waitForLoadState('networkidle');
   // or
   await page.waitForTimeout(2000);
   ```

2. **Handle dynamic content**
   - Products may take time to load
   - Use `waitForSelector` before interacting
   - Check for "product is loading" placeholders

3. **Error handling**
   - Check if store location needs updating
   - Verify cart updates after adding items
   - Handle modal dialogs (store selection, age verification, etc.)

4. **Search tips**
   - Clear search box before new search
   - Use trending searches for common items
   - Check for coupons/deals on products

### Common Tasks

#### Task: Add Multiple Items
```javascript
// Search and add milk
await page.fill('searchbox', 'milk');
await page.press('searchbox', 'Enter');
await page.waitForSelector('text="results for"');
await page.click('button:has-text("Add to cart"):first');

// Search and add bread
await page.fill('searchbox', 'bread');
await page.press('searchbox', 'Enter');
await page.waitForSelector('text="results for"');
await page.click('button:has-text("Add to cart"):first');
```

#### Task: Filter/Sort Results
```javascript
// Open sort dropdown
await page.click('combobox[aria-label="Sort by"]');
await page.click('option:has-text("Price: Low to High")');

// Use filters
await page.click('button:has-text("All Filters")');
// Handle filter modal
```

#### Task: Check Weekly Deals
```javascript
await page.click('link:has-text("Weekly Ad")');
// or navigate directly
await page.goto('https://giantfoodstores.com/weekly-ad');
```

### Authentication (if needed)
```javascript
// Login flow (when required for checkout)
await page.click('button:has-text("Sign In")');
await page.fill('input[type="email"]', process.env.EMAIL);
await page.fill('input[type="password"]', process.env.PASSWORD);
await page.click('button:has-text("Sign In"):visible');
```

### Notes
- The site uses lazy loading for products - scroll may be needed for more results
- Sponsored products are marked but function the same as regular products
- Prices and availability may vary by store location
- Some features may require login (saved lists, previous orders, checkout)

## Shopping List Validation

### After Building Cart - ALWAYS CHECK:
Reference the `weekly-staples-list.md` file and verify the following items are in the cart or intentionally skipped:

#### Core Staples (Should be in every order unless user says otherwise):
- **Produce:** Cucumber, Cilantro, Limes (1-2), Chiquita Bananas, Green Onions
- **Dairy:** Icelandic Provisions Vanilla Skyr (3-5), Silk Oatmeal Cookie Creamer (1-2)
- **Deli:** Cooper Sharp White American Cheese (~0.5 lb)
- **Water:** Deer Park Exchange (2-4 units)
- **Kids:** Danimals 12-pk, Gerber Lil' Crunchies, Happy Baby Yogis

#### Frequent Items to Prompt About:
- Nature's Promise 99% Lean Ground Chicken
- Cracker Barrel Extra Sharp Cheddar Cracker Cuts
- Jimmy Dean Delights Turkey Sausage Sandwiches
- Giovanni Rana Mozzarella Ravioli
- Eggs
- Strawberries (if in season)

### Validation Prompts:
After cart is built, remind user:
1. "I've added [X items]. Let me check against your weekly staples..."
2. "I notice we don't have [missing staple items]. Should I add them?"
3. "Your usual order includes [X units] of Deer Park water. Currently have [Y]. Adjust?"

### Reference Files:
- `weekly-staples-list.md` - Checklist of always/frequent items
- `preferred-items-reference.md` - Brand and specification preferences
- `grocery-orders-history.md` - Recent order patterns
- `periodic-household-items.md` - Household staples to check periodically

### Periodic Items Check:
For orders marked as "big shop" or every 3-4 orders, ask about:
1. **Paper/Cleaning:** "How are you on paper towels, toilet paper, and cleaning supplies?"
2. **Kitchen Basics:** "Need any cooking oils, spices, or baking items?"
3. **Fresh Basics:** "Need regular onions, potatoes, or butter?"
4. **Condiments:** "Running low on any condiments or sauces?"

Priority items most often forgotten:
- Paper towels, toilet paper, trash bags
- Butter, olive oil, onions
- Dish soap, laundry detergent
- Coffee

## Recipe Storage with Qdrant

### Collection Setup
This project uses a Qdrant collection called **`food_recipes`** for storing recipes and ingredients.

### Storing New Recipes
When the user provides a recipe URL:
1. **First time seeing a recipe:**
   - Fetch the recipe using WebFetch or browser tools
   - Extract ingredients, quantities, and cooking notes
   - Store in the `food_recipes` collection with metadata (URL, cuisine, protein type, etc.)
   - Note which ingredients aren't available at Giant

2. **Recipe already stored:**
   - Search the `food_recipes` collection first
   - If found, use the cached version instead of fetching again

### Using the Recipe Collection
**IMPORTANT:** Always use collection name `food_recipes` when interacting with recipe data:
```
# Storing recipes
Store in the food_recipes collection: [recipe data]

# Searching recipes
Search the food_recipes collection for: bulgogi burgers

# Example queries
- "I want to make bulgogi burgers and sambal noodles this week"
- "What recipes do I have saved?"
- "Show me recipes with tofu"
```

### Shopping List Generation
When user wants to cook saved recipes:
1. Search `food_recipes` collection for the requested recipes
2. Extract all ingredients from the stored data
3. Cross-reference with `weekly-staples-list.md` and `last-order-items.md`
4. **Check Pantry Staples** (see section below)
5. Generate consolidated shopping list with:
   - Items available at Giant
   - Items needing specialty stores
   - Quantities adjusted for multiple recipes

### Pantry Staples Intelligence
**IMPORTANT:** These items are typically bought in larger quantities and last weeks/months. Don't automatically add them - ASK FIRST.

#### Long-Lasting Condiments & Sauces
Items to **confirm before adding**:
- **Refrigerated:** Mayonnaise (Hellmann's), Ketchup, Yellow mustard, BBQ sauce, Hot sauce/Sriracha
- **Asian:** Soy sauce, Rice vinegar, Sesame oil, Sambal oelek, Gochujang
- **Oils/Vinegars:** Olive oil, Vegetable oil, Balsamic vinegar, Apple cider vinegar
- **Other:** Honey, Peanut butter (Skippy), Jelly/Jam

**How to ask:** "The recipe calls for [soy sauce, mayo, etc.]. I assume you have these pantry staples, but let me know if you need any: [list items]"

#### Baking & Dry Goods
Items that last months - **always confirm need**:
- **Basics:** All-purpose flour, Sugar (white/brown), Baking soda, Baking powder
- **Seasonings:** Salt, Black pepper, Garlic powder, Onion powder, Italian seasoning
- **Extracts:** Vanilla extract
- **Pasta/Rice:** Dried pasta, Rice (check type needed)

**How to ask:** "For baking/cooking basics, do you need me to add any: flour, sugar, baking powder, salt, pepper?"

#### Fresh Items That Keep Well
**Ask about these less frequently** (every 2-3 orders):
- Butter (keeps for weeks)
- Garlic bulbs (last 3-4 weeks)
- Yellow/white onions (last 2-3 weeks)
- Potatoes (last 2-3 weeks)

#### Smart Prompting Rules
1. **For recipe ingredients:** Check if item is a pantry staple first
2. **Group confirmations:** "I see the recipes need soy sauce, sesame oil, and rice vinegar. These are pantry staples - need any?"
3. **Reference last purchase:** Check `last-order-items.md` and `periodic-household-items.md`
4. **Frequency guidelines:**
   - Weekly staples: Always add (per `weekly-staples-list.md`)
   - Pantry condiments: Ask every 4-6 weeks
   - Baking supplies: Ask every 6-8 weeks
   - Spices: Ask every 2-3 months

#### Example Interaction
```
User: "I want to make stir fry this week"

Claude: "For the stir fry, I'll add:
✓ Fresh vegetables and protein

The recipe also uses these pantry items - let me know if you need any:
- Soy sauce
- Sesame oil
- Garlic
- Ginger

Should I add any of these pantry items, or do you have them?"
```

#### Items to ALWAYS Add (Unless User Says Otherwise)
Even though these keep well, user buys them frequently:
- Eggs (per shopping pattern)
- Milk/Creamer (Silk Oatmilk Creamer)
- Bread (goes stale quickly)
- Fresh produce from `weekly-staples-list.md`

### Recipe Metadata Structure
Each stored recipe should include:
- URL (source)
- Full ingredients list with quantities
- Shopping notes (what's available at Giant vs specialty stores)
- Cuisine type
- Main protein
- Cook time
- Any substitutions made

## Environment Variables
Login credentials are stored in `.env` file in this directory.
- The `.env` file contains EMAIL and PASSWORD for Giant Food Stores authentication
- Default store ZIP: 19355 (Malvern, PA)
- **IMPORTANT:** Always use credentials from `.env` file for login automation
- Never hardcode credentials in scripts or logs