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

#[test]
fn test_consolidate_no_duplicates() -> TestResult {
    let items = vec![
        make_item("Cucumber", 1, Category::Produce, ItemSource::Staple),
        make_item("Garlic", 1, Category::Produce, ItemSource::Recipe("Test".to_string())),
    ];

    let result = consolidate(items);
    assert_eq!(result.len(), 2);
    Ok(())
}

#[test]
fn test_consolidate_merges_duplicates() -> TestResult {
    let items = vec![
        make_item("Cilantro Fresh", 1, Category::Produce, ItemSource::Staple),
        make_item("Cilantro Fresh", 1, Category::Produce, ItemSource::Recipe("Noodles".to_string())),
    ];

    let result = consolidate(items);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "Cilantro Fresh");
    Ok(())
}

#[test]
fn test_consolidate_case_insensitive() -> TestResult {
    let items = vec![
        make_item("English Cucumber", 1, Category::Produce, ItemSource::Staple),
        make_item("english cucumber", 1, Category::Produce, ItemSource::Recipe("Test".to_string())),
    ];

    let result = consolidate(items);
    assert_eq!(result.len(), 1);
    Ok(())
}

#[test]
fn test_consolidate_staple_takes_priority() -> TestResult {
    let items = vec![
        make_item("Garlic", 1, Category::Produce, ItemSource::Recipe("Test".to_string())),
        make_item("Garlic", 1, Category::Produce, ItemSource::Staple),
    ];

    let result = consolidate(items);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].source, ItemSource::Staple);
    Ok(())
}

#[test]
fn test_consolidate_takes_max_quantity() -> TestResult {
    let items = vec![
        make_item("Yogurt", 3, Category::Dairy, ItemSource::Staple),
        make_item("Yogurt", 1, Category::Dairy, ItemSource::Recipe("Test".to_string())),
    ];

    let result = consolidate(items);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].quantity, 3);
    Ok(())
}

#[test]
fn test_consolidate_combines_notes() -> TestResult {
    let mut item1 = make_item("Chicken", 1, Category::Meat, ItemSource::Staple);
    item1.note = Some("99% lean".to_string());

    let mut item2 = make_item("Chicken", 1, Category::Meat, ItemSource::Recipe("Test".to_string()));
    item2.note = Some("for meatballs".to_string());

    let result = consolidate(vec![item1, item2]);
    assert_eq!(result.len(), 1);
    let note = result[0].note.as_deref().unwrap_or("");
    assert!(note.contains("99% lean"));
    assert!(note.contains("for meatballs"));
    Ok(())
}

#[test]
fn test_consolidate_empty_input() -> TestResult {
    let result = consolidate(vec![]);
    assert!(result.is_empty());
    Ok(())
}

#[test]
fn test_consolidate_sorted_by_category_then_name() -> TestResult {
    let items = vec![
        make_item("Yogurt", 1, Category::Dairy, ItemSource::Staple),
        make_item("Banana", 1, Category::Produce, ItemSource::Staple),
        make_item("Apple", 1, Category::Produce, ItemSource::Staple),
        make_item("Chicken", 1, Category::Meat, ItemSource::Staple),
    ];

    let result = consolidate(items);
    // Dairy < Meat < Produce (alphabetical by Debug format)
    assert_eq!(result[0].category, Category::Dairy);
    assert_eq!(result[1].category, Category::Meat);
    // Produce items sorted by name
    assert_eq!(result[2].name, "Apple");
    assert_eq!(result[3].name, "Banana");
    Ok(())
}
