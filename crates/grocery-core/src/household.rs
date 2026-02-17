use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{GroceryError, GroceryResult};
use crate::types::{Category, FrequencyTier, Store};

/// Top-level household model parsed from household-model.yaml.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseholdModel {
    pub family: FamilyProfile,
    pub stores: Stores,
    #[serde(default)]
    pub amazon_recurring: Vec<AmazonItem>,
    #[serde(default)]
    pub giant_recurring: Vec<GiantItem>,
    #[serde(default)]
    pub acme_recurring: Vec<serde_yaml::Value>,
    #[serde(default)]
    pub meal_plan_source: Option<String>,
}

impl HouseholdModel {
    /// Load from a YAML file path.
    pub fn from_file(path: &Path) -> GroceryResult<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_yaml(&content)
    }

    /// Parse from a YAML string.
    pub fn from_yaml(yaml: &str) -> GroceryResult<Self> {
        serde_yaml::from_str(yaml).map_err(|e| GroceryError::HouseholdParse(e.to_string()))
    }

    /// Get all Giant recurring items at a given tier.
    pub fn giant_items_by_tier(&self, tier: FrequencyTier) -> Vec<&GiantItem> {
        self.giant_recurring
            .iter()
            .filter(|item| item.tier() == tier)
            .collect()
    }

    /// Get all tier 1 (every order) Giant items.
    pub fn staples(&self) -> Vec<&GiantItem> {
        self.giant_items_by_tier(FrequencyTier::EveryOrder)
    }
}

/// Family members.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyProfile {
    pub members: Vec<FamilyMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMember {
    pub name: String,
    #[serde(default)]
    pub age: Option<u8>,
}

/// Store configurations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stores {
    #[serde(default)]
    pub giant: Option<StoreConfig>,
    #[serde(default)]
    pub acme: Option<StoreConfig>,
    #[serde(default)]
    pub amazon: Option<StoreConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreConfig {
    #[serde(rename = "type")]
    pub store_type: Option<String>,
    #[serde(default)]
    pub order_method: Option<String>,
    #[serde(default)]
    pub frequency: Option<String>,
    #[serde(default)]
    pub account: Option<String>,
    #[serde(default)]
    pub store: Option<String>,
    #[serde(default)]
    pub avg_order_total: Option<String>,
    #[serde(default)]
    pub total_annual_spend: Option<String>,
    #[serde(default)]
    pub avg_items_per_order: Option<u32>,
    #[serde(default)]
    pub data_sources: Option<Vec<String>>,
    #[serde(default)]
    pub data_source: Option<String>,
}

/// Amazon recurring item from household model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmazonItem {
    pub item: String,
    pub category: Category,
    #[serde(default)]
    pub cycle_days: Option<String>,
    #[serde(default)]
    pub last_seen: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

/// Giant recurring item from household model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiantItem {
    pub item: String,
    pub category: Category,
    #[serde(default)]
    pub frequency: Option<String>,
    #[serde(default)]
    pub typical_qty: Option<String>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub store: Option<Store>,
    #[serde(default)]
    pub oos_count: Option<u32>,
    #[serde(default)]
    pub note: Option<String>,
}

impl GiantItem {
    /// Parse frequency string like "14/18" into a tier.
    pub fn tier(&self) -> FrequencyTier {
        let Some(freq) = &self.frequency else {
            return FrequencyTier::Rare;
        };
        parse_frequency_tier(freq)
    }
}

/// Parse "14/18" style frequency strings into a tier.
fn parse_frequency_tier(freq: &str) -> FrequencyTier {
    let parts: Vec<&str> = freq.split('/').collect();
    if parts.len() != 2 {
        return FrequencyTier::Rare;
    }
    let appearances = parts[0].trim().parse::<u8>().unwrap_or(0);
    let total = parts[1].trim().parse::<u8>().unwrap_or(0);
    FrequencyTier::from_frequency(appearances, total)
}

#[cfg(test)]
#[path = "household_tests.rs"]
mod tests;
