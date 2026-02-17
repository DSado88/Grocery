use serde::{Deserialize, Serialize};

/// Grocery store identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Store {
    Giant,
    Acme,
    Amazon,
    TraderJoes,
    Other(String),
}

/// Product category matching Giant's store layout.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Produce,
    Dairy,
    Meat,
    Deli,
    Frozen,
    Canned,
    Bread,
    Pasta,
    Beverages,
    Snacks,
    Condiments,
    Baking,
    Breakfast,
    Baby,
    Household,
    Health,
    Personal,
    Pet,
    Other(String),
}

/// Frequency tier based on purchase history (N out of 18 Giant orders).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FrequencyTier {
    /// Tier 1: 12-18/18 orders — auto-add to every list
    EveryOrder,
    /// Tier 2: 7-11/18 orders — add most weeks
    MostOrders,
    /// Tier 3: 3-6/18 orders — add as needed
    Occasional,
    /// Below threshold: appeared fewer than 3 times
    Rare,
}

impl FrequencyTier {
    /// Classify an item based on how many of 18 orders it appeared in.
    pub fn from_frequency(appearances: u8, total_orders: u8) -> Self {
        if total_orders == 0 {
            return Self::Rare;
        }
        let ratio = f64::from(appearances) / f64::from(total_orders);
        // Boundaries: 12/18, 7/18, 3/18
        if ratio >= 12.0 / 18.0 {
            Self::EveryOrder
        } else if ratio >= 7.0 / 18.0 {
            Self::MostOrders
        } else if ratio >= 3.0 / 18.0 {
            Self::Occasional
        } else {
            Self::Rare
        }
    }
}

/// Primary protein in a recipe.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Protein {
    GroundChicken,
    Tofu,
    Vegetarian,
    GroundBeef,
    Tuna,
    ChickenBreast,
    ChickenThigh,
    Eggs,
    Pork,
    Salmon,
    Shrimp,
    Steak,
    Ham,
    Sausage,
    Haddock,
    Lamb,
    Other(String),
}

/// Cuisine type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cuisine {
    SoutheastAsian,
    Korean,
    Japanese,
    Mexican,
    Chinese,
    ItalianPasta,
    Mediterranean,
    AmericanComfort,
    MiddleEastern,
    Indian,
    Thai,
    French,
    General,
    Other(String),
}

#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;
