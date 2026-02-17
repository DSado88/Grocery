use std::collections::HashMap;

use grocery_core::item::{ItemSource, ShoppingItem};

/// Deduplicate and merge shopping items by canonical name.
///
/// Items with the same name (case-insensitive) are merged:
/// - Quantity: max of existing and new
/// - Source: Staple takes priority over Recipe
/// - Notes: combined
///
/// Results are sorted by category then name.
pub fn consolidate(items: Vec<ShoppingItem>) -> Vec<ShoppingItem> {
    let mut map: HashMap<String, ShoppingItem> = HashMap::new();

    for item in items {
        let key = item.name.to_lowercase();

        if let Some(existing) = map.get_mut(&key) {
            // Merge quantities (take the max)
            if item.quantity > existing.quantity {
                existing.quantity = item.quantity;
            }

            // Staple source takes priority
            if item.source == ItemSource::Staple {
                existing.source = ItemSource::Staple;
            }

            // Combine notes
            if let Some(new_note) = &item.note {
                match &mut existing.note {
                    Some(existing_note) => {
                        existing_note.push_str("; ");
                        existing_note.push_str(new_note);
                    }
                    None => {
                        existing.note = Some(new_note.clone());
                    }
                }
            }
        } else {
            map.insert(key, item);
        }
    }

    let mut result: Vec<ShoppingItem> = map.into_values().collect();
    result.sort_by(|a, b| {
        let cat_a = format!("{:?}", a.category);
        let cat_b = format!("{:?}", b.category);
        cat_a.cmp(&cat_b).then(a.name.cmp(&b.name))
    });

    result
}

#[cfg(test)]
#[path = "consolidator_tests.rs"]
mod tests;
