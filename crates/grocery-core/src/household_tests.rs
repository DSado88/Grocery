use super::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_parse_household_model_from_real_yaml() -> TestResult {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("household-model.yaml"))
        .ok_or("could not resolve household-model.yaml path")?;

    if !path.exists() {
        // Skip if running outside the workspace root
        return Ok(());
    }

    let model = HouseholdModel::from_file(&path)?;

    // Family should have 4 members (David, Christine, Daughter, Dog)
    assert_eq!(model.family.members.len(), 4);
    assert_eq!(model.family.members[0].name, "David");
    assert_eq!(model.family.members[1].name, "Christine");

    // Should have Giant recurring items
    assert!(!model.giant_recurring.is_empty());

    // Should have Amazon recurring items
    assert!(!model.amazon_recurring.is_empty());

    // Giant store config should exist
    let giant = model.stores.giant.as_ref().ok_or("missing giant store config")?;
    assert_eq!(giant.store_type.as_deref(), Some("grocery"));

    Ok(())
}

#[test]
fn test_giant_items_by_tier() -> TestResult {
    let yaml = r#"
family:
  members:
    - name: Test
stores:
  giant:
    type: grocery
giant_recurring:
  - item: "Always Buy Item"
    category: dairy
    frequency: "16/18"
    price: "$5.99"
  - item: "Sometimes Item"
    category: produce
    frequency: "9/18"
  - item: "Occasional Item"
    category: meat
    frequency: "4/18"
  - item: "Rare Item"
    category: snacks
    frequency: "1/18"
"#;
    let model = HouseholdModel::from_yaml(yaml)?;
    assert_eq!(model.giant_recurring.len(), 4);

    let staples = model.staples();
    assert_eq!(staples.len(), 1);
    assert_eq!(staples[0].item, "Always Buy Item");

    let tier2 = model.giant_items_by_tier(FrequencyTier::MostOrders);
    assert_eq!(tier2.len(), 1);
    assert_eq!(tier2[0].item, "Sometimes Item");

    let tier3 = model.giant_items_by_tier(FrequencyTier::Occasional);
    assert_eq!(tier3.len(), 1);
    assert_eq!(tier3[0].item, "Occasional Item");

    Ok(())
}

#[test]
fn test_parse_frequency_tier_various_formats() -> TestResult {
    assert_eq!(parse_frequency_tier("18/18"), FrequencyTier::EveryOrder);
    assert_eq!(parse_frequency_tier("12/18"), FrequencyTier::EveryOrder);
    assert_eq!(parse_frequency_tier("11/18"), FrequencyTier::MostOrders);
    assert_eq!(parse_frequency_tier("7/18"), FrequencyTier::MostOrders);
    assert_eq!(parse_frequency_tier("6/18"), FrequencyTier::Occasional);
    assert_eq!(parse_frequency_tier("3/18"), FrequencyTier::Occasional);
    assert_eq!(parse_frequency_tier("2/18"), FrequencyTier::Rare);
    assert_eq!(parse_frequency_tier("invalid"), FrequencyTier::Rare);
    assert_eq!(parse_frequency_tier(""), FrequencyTier::Rare);
    Ok(())
}

#[test]
fn test_giant_item_without_frequency_is_rare() -> TestResult {
    let yaml = r#"
family:
  members: []
stores: {}
giant_recurring:
  - item: "No Frequency Item"
    category: dairy
"#;
    let model = HouseholdModel::from_yaml(yaml)?;
    assert_eq!(model.giant_recurring[0].tier(), FrequencyTier::Rare);
    Ok(())
}
