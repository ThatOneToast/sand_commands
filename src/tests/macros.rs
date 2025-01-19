
use crate::{prelude::MinecraftEntity, target_filter, types::world::Distance};

#[test]
fn test_target_filter_macro() {
    let filter = target_filter!(@MinecraftEntity, distance = "5..10", team = "Red", limit = "1");

    assert_eq!(
        matches!(filter.distance, Some(Distance::Range(5.0, 10.0))),
        true
    );
    assert_eq!(filter.team, Some("Red".to_string()));
    assert_eq!(filter.limit, Some(1));

    // Test different distance formats
    let exact_distance = target_filter!(@MinecraftEntity, distance = "10");
    assert_eq!(
        matches!(exact_distance.distance, Some(Distance::Exact(10.0))),
        true
    );

    let min_distance = target_filter!(@MinecraftEntity, distance = "5..");
    assert_eq!(
        matches!(min_distance.distance, Some(Distance::Min(5.0))),
        true
    );

    let max_distance = target_filter!(@MinecraftEntity, distance = "..10");
    assert_eq!(
        matches!(max_distance.distance, Some(Distance::Max(10.0))),
        true
    );
}