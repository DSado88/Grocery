use super::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_frequency_tier_every_order() -> TestResult {
    // 14/18 = 0.78 → EveryOrder (≥0.67)
    assert_eq!(FrequencyTier::from_frequency(14, 18), FrequencyTier::EveryOrder);
    // 12/18 = 0.67 → EveryOrder (boundary)
    assert_eq!(FrequencyTier::from_frequency(12, 18), FrequencyTier::EveryOrder);
    Ok(())
}

#[test]
fn test_frequency_tier_most_orders() -> TestResult {
    // 11/18 = 0.61 → MostOrders
    assert_eq!(FrequencyTier::from_frequency(11, 18), FrequencyTier::MostOrders);
    // 7/18 = 0.39 → MostOrders (boundary)
    assert_eq!(FrequencyTier::from_frequency(7, 18), FrequencyTier::MostOrders);
    Ok(())
}

#[test]
fn test_frequency_tier_occasional() -> TestResult {
    // 6/18 = 0.33 → Occasional
    assert_eq!(FrequencyTier::from_frequency(6, 18), FrequencyTier::Occasional);
    // 3/18 = 0.17 → Occasional (boundary)
    assert_eq!(FrequencyTier::from_frequency(3, 18), FrequencyTier::Occasional);
    Ok(())
}

#[test]
fn test_frequency_tier_rare() -> TestResult {
    // 2/18 = 0.11 → Rare
    assert_eq!(FrequencyTier::from_frequency(2, 18), FrequencyTier::Rare);
    // 0/18 → Rare
    assert_eq!(FrequencyTier::from_frequency(0, 18), FrequencyTier::Rare);
    Ok(())
}

#[test]
fn test_frequency_tier_zero_total_orders() -> TestResult {
    // Edge case: zero total orders → Rare
    assert_eq!(FrequencyTier::from_frequency(5, 0), FrequencyTier::Rare);
    Ok(())
}

#[test]
fn test_category_serde_roundtrip() -> TestResult {
    let cat = Category::Produce;
    let json = serde_json::to_string(&cat)?;
    let parsed: Category = serde_json::from_str(&json)?;
    assert_eq!(parsed, cat);
    Ok(())
}

#[test]
fn test_store_serde_roundtrip() -> TestResult {
    let store = Store::Giant;
    let json = serde_json::to_string(&store)?;
    let parsed: Store = serde_json::from_str(&json)?;
    assert_eq!(parsed, store);
    Ok(())
}
