use std::collections::BTreeMap;
use std::fmt::Write;

use grocery_core::error::GroceryResult;
use grocery_core::item::{ItemSource, ShoppingItem};

/// A finalized shopping list ready for output.
#[derive(Debug, Clone)]
pub struct ShoppingList {
    pub items: Vec<ShoppingItem>,
}

impl ShoppingList {
    /// Create a new shopping list from consolidated items.
    pub fn new(items: Vec<ShoppingItem>) -> Self {
        Self { items }
    }

    /// Total number of items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Group items by category name.
    pub fn by_category(&self) -> BTreeMap<String, Vec<&ShoppingItem>> {
        let mut groups: BTreeMap<String, Vec<&ShoppingItem>> = BTreeMap::new();
        for item in &self.items {
            let key = category_display(&item.category);
            groups.entry(key).or_default().push(item);
        }
        groups
    }

    /// Format as human-readable text with markdown checkboxes.
    pub fn format_text(&self) -> String {
        let mut out = String::new();
        let groups = self.by_category();

        for (category, items) in &groups {
            let _ = writeln!(out, "## {category}");
            for item in items {
                let source = source_label(&item.source);
                let _ = writeln!(out, "- [ ] {} ({}) [{}]", item.name, item.quantity, source);
            }
            out.push('\n');
        }

        out
    }

    /// Format as JSON array.
    pub fn format_json(&self) -> GroceryResult<String> {
        let json = serde_json::to_string_pretty(&self.items)?;
        Ok(json)
    }

    /// Format as compact text for iMessage (fits in one message).
    pub fn format_compact(&self) -> String {
        let mut out = format!("Shopping List ({} items)\n", self.items.len());
        let groups = self.by_category();

        for (category, items) in &groups {
            let upper = category.to_uppercase();
            let names: Vec<String> = items
                .iter()
                .map(|item| {
                    if item.quantity > 1 {
                        format!("{} x{}", item.name, item.quantity)
                    } else {
                        item.name.clone()
                    }
                })
                .collect();
            let _ = writeln!(out, "{upper}: {}", names.join(", "));
        }

        out
    }
}

/// Human-readable category name.
fn category_display(category: &grocery_core::types::Category) -> String {
    use grocery_core::types::Category;
    match category {
        Category::Produce => "Produce".to_string(),
        Category::Dairy => "Dairy".to_string(),
        Category::Meat => "Meat".to_string(),
        Category::Deli => "Deli".to_string(),
        Category::Frozen => "Frozen".to_string(),
        Category::Canned => "Canned".to_string(),
        Category::Bread => "Bread".to_string(),
        Category::Pasta => "Pasta".to_string(),
        Category::Beverages => "Beverages".to_string(),
        Category::Snacks => "Snacks".to_string(),
        Category::Condiments => "Condiments".to_string(),
        Category::Baking => "Baking".to_string(),
        Category::Breakfast => "Breakfast".to_string(),
        Category::Baby => "Baby".to_string(),
        Category::Household => "Household".to_string(),
        Category::Health => "Health".to_string(),
        Category::Personal => "Personal".to_string(),
        Category::Pet => "Pet".to_string(),
        Category::Other(s) => s.clone(),
    }
}

/// Short label for item source.
fn source_label(source: &ItemSource) -> String {
    match source {
        ItemSource::Staple => "staple".to_string(),
        ItemSource::Recipe(name) => name.clone(),
        ItemSource::UserRequest => "requested".to_string(),
        ItemSource::FrequencyTrigger => "frequency".to_string(),
    }
}

#[cfg(test)]
#[path = "formatter_tests.rs"]
mod tests;
