use super::*;
use grocery_core::item::{ItemSource, ShoppingItem};
use grocery_core::types::Category;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn make_item(name: &str, qty: u32, category: Category, source: ItemSource) -> ShoppingItem {
    ShoppingItem {
        name: name.to_string(),
        quantity: qty,
        category,
        source,
        note: None,
    }
}

fn sample_list() -> ShoppingList {
    ShoppingList::new(vec![
        make_item("English Cucumber", 1, Category::Produce, ItemSource::Staple),
        make_item("Cilantro", 1, Category::Produce, ItemSource::Staple),
        make_item("Garlic", 1, Category::Produce, ItemSource::Recipe("Sambal Noodles".to_string())),
        make_item("Yogurt Cup", 3, Category::Dairy, ItemSource::Staple),
        make_item("Ground Chicken", 1, Category::Meat, ItemSource::Staple),
    ])
}

#[test]
fn test_shopping_list_len() -> TestResult {
    let list = sample_list();
    assert_eq!(list.len(), 5);
    assert!(!list.is_empty());
    Ok(())
}

#[test]
fn test_shopping_list_empty() -> TestResult {
    let list = ShoppingList::new(vec![]);
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    Ok(())
}

#[test]
fn test_by_category_groups_correctly() -> TestResult {
    let list = sample_list();
    let groups = list.by_category();

    assert_eq!(groups.len(), 3); // Produce, Dairy, Meat
    assert_eq!(groups.get("Produce").map(|v| v.len()), Some(3));
    assert_eq!(groups.get("Dairy").map(|v| v.len()), Some(1));
    assert_eq!(groups.get("Meat").map(|v| v.len()), Some(1));
    Ok(())
}

#[test]
fn test_format_text_contains_categories() -> TestResult {
    let list = sample_list();
    let text = list.format_text();

    assert!(text.contains("## Produce"));
    assert!(text.contains("## Dairy"));
    assert!(text.contains("## Meat"));
    Ok(())
}

#[test]
fn test_format_text_contains_items() -> TestResult {
    let list = sample_list();
    let text = list.format_text();

    assert!(text.contains("English Cucumber"));
    assert!(text.contains("[staple]"));
    assert!(text.contains("[Sambal Noodles]"));
    assert!(text.contains("Yogurt Cup (3)"));
    Ok(())
}

#[test]
fn test_format_text_has_checkboxes() -> TestResult {
    let list = sample_list();
    let text = list.format_text();

    let checkbox_count = text.matches("- [ ]").count();
    assert_eq!(checkbox_count, 5);
    Ok(())
}

#[test]
fn test_format_json_is_valid() -> TestResult {
    let list = sample_list();
    let json = list.format_json()?;

    let parsed: Vec<ShoppingItem> = serde_json::from_str(&json)?;
    assert_eq!(parsed.len(), 5);
    assert_eq!(parsed[0].name, "English Cucumber");
    Ok(())
}

#[test]
fn test_format_compact_header() -> TestResult {
    let list = sample_list();
    let compact = list.format_compact();

    assert!(compact.starts_with("Shopping List (5 items)"));
    Ok(())
}

#[test]
fn test_format_compact_categories_uppercase() -> TestResult {
    let list = sample_list();
    let compact = list.format_compact();

    assert!(compact.contains("PRODUCE:"));
    assert!(compact.contains("DAIRY:"));
    assert!(compact.contains("MEAT:"));
    Ok(())
}

#[test]
fn test_format_compact_quantity_notation() -> TestResult {
    let list = sample_list();
    let compact = list.format_compact();

    // Items with qty > 1 should show "x3"
    assert!(compact.contains("Yogurt Cup x3"));
    // Items with qty 1 should NOT show "x1"
    assert!(!compact.contains("English Cucumber x1"));
    Ok(())
}

#[test]
fn test_format_empty_list() -> TestResult {
    let list = ShoppingList::new(vec![]);
    let text = list.format_text();
    assert!(text.is_empty() || text.trim().is_empty());

    let compact = list.format_compact();
    assert!(compact.contains("0 items"));

    let json = list.format_json()?;
    assert_eq!(json.trim(), "[]");
    Ok(())
}
