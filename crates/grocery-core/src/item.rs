use serde::{Deserialize, Serialize};

use crate::types::{Category, FrequencyTier, Store};

/// A recurring grocery item from the household model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringItem {
    pub item: String,
    pub category: Category,
    pub store: Store,
    pub tier: FrequencyTier,
    #[serde(default)]
    pub frequency: Option<String>,
    #[serde(default)]
    pub typical_qty: Option<String>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub oos_count: Option<u32>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub cycle_days: Option<String>,
    #[serde(default)]
    pub last_seen: Option<String>,
}

/// An item on a shopping list (generated, not from the model).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShoppingItem {
    pub name: String,
    pub quantity: u32,
    pub category: Category,
    pub source: ItemSource,
    #[serde(default)]
    pub note: Option<String>,
}

/// Where a shopping list item came from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemSource {
    /// From a recipe's ingredient list
    Recipe(String),
    /// Weekly staple (auto-added)
    Staple,
    /// User explicitly requested
    UserRequest,
    /// Household model frequency trigger
    FrequencyTrigger,
}
