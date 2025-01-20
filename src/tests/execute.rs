use crate::{
    assert_validation, execute,
    prelude::{
        execute::{
            ExecuteAnchor, ExecuteBossBarValue, ExecuteConditionCategory, ExecuteDataCommand,
            ExecuteFacing, ExecuteItemsCommand, ExecutePositioned, ExecuteScanMode,
            ExecuteScoreCommand, ExecuteStoreAction, ExecuteStoreCommand, ExecuteSubCommand,
        },
        Execute, Kill,
    },
    types::{
        HeightMap, Location, MinecraftComparative, MinecraftDimensions, MinecraftType,
        TargetSelector,
    },
};
use std::sync::Arc;

// Tests for condition categories
#[tokio::test]
async fn if_biome_condition() {
    let command = execute!(
        if ExecuteConditionCategory::BIOME {
            pos: Location {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            biome: "minecraft:plains".to_string(),
        } => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

#[tokio::test]
async fn unless_block_condition() {
    let command = execute!(
        unless ExecuteConditionCategory::BLOCK {
            pos: Location {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            predicate: "minecraft:stone".to_string(),
        } => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

#[tokio::test]
async fn if_blocks_condition() {
    let command = execute!(
        if ExecuteConditionCategory::BLOCKS {
            start: Location {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            end: Location {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            destination: Location {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            },
            scan_mode: ExecuteScanMode::ALL,
        } => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

#[tokio::test]
async fn if_data_block_condition() {
    let command = execute!(
        if ExecuteConditionCategory::DATA(
            ExecuteDataCommand::BLOCK {
                pos: Location::new(0.0, 0.0, 0.0),
                nbt_path: "Items[0]".to_string(),
            }
        ) => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

#[tokio::test]
async fn if_dimension_condition() {
    let command = execute!(
        if ExecuteConditionCategory::DIMENSION(MinecraftDimensions::Overworld)
        => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

#[tokio::test]
async fn if_entity_condition() {
    let command = execute!(
        if ExecuteConditionCategory::ENTITY(TargetSelector::Current(None))
        => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

#[tokio::test]
async fn if_function_condition() {
    let command = execute!(
        if ExecuteConditionCategory::FUNCTION("minecraft:test_function".to_string())
        => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

#[tokio::test]
async fn if_items_condition() {
    let command = execute!(
        if ExecuteConditionCategory::ITEMS(
            ExecuteItemsCommand::BLOCK {
                source_pos: Location::new(0.0, 0.0, 0.0),
                slots: "container.0".to_string(),
                predicate: "minecraft:diamond".to_string(),
            }
        ) => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

#[tokio::test]
async fn if_loaded_condition() {
    let command = execute!(
        if ExecuteConditionCategory::LOADED(Location::new(0.0, 0.0, 0.0))
        => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}


#[tokio::test]
async fn if_score_matches_condition() {
    let command = execute!(
        if ExecuteConditionCategory::SCORE(
            ExecuteScoreCommand::MATCHES {
                target: TargetSelector::Current(None),
                objective: "points".to_string(),
                range: "1..5".to_string(),
            }
        ) => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

#[tokio::test]
async fn if_score_compare_condition() {
    let command = execute!(
        if ExecuteConditionCategory::SCORE(
            ExecuteScoreCommand::COMPARE {
                target: TargetSelector::Current(None),
                objective: "points".to_string(),
                comparative: MinecraftComparative::GreaterThan,
                source: TargetSelector::Other("Toast".to_string()),
                source_objective: "threshold".to_string(),
            }
        ) => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

// Tests for positioned variants
#[tokio::test]
async fn positioned_as_target() {
    let command = execute!(
        positioned ExecutePositioned::TARGETS(TargetSelector::Current(None))
        => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

#[tokio::test]
async fn positioned_over_heightmap() {
    let command = execute!(
        positioned ExecutePositioned::HEIGHTMAP(HeightMap::MotionBlocking)
        => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

// Tests for store variants
#[tokio::test]
async fn store_block_result() {
    let command = execute!(
        store ExecuteStoreAction::RESULT,
        ExecuteStoreCommand::BLOCK {
            target_pos: Location::new(0.0, 0.0, 0.0),
            nbt_path: "Items[0].Count".to_string(),
            data_type: MinecraftType::BYTE,
            scale: 1.0,
        } => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

#[tokio::test]
async fn store_bossbar_value() {
    let command = execute!(
        store ExecuteStoreAction::RESULT,
        ExecuteStoreCommand::BOSSBAR {
            resource_location: "minecraft:boss".to_string(),
            value: ExecuteBossBarValue::VALUE,
        } => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

#[tokio::test]
async fn store_entity_success() {
    let command = execute!(
        store ExecuteStoreAction::SUCCESS,
        ExecuteStoreCommand::ENTITY {
            target: TargetSelector::Current(None),
            nbt_path: "Health".to_string(),
            data_type: MinecraftType::FLOAT,
            scale: 2.0,
        } => run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

// Complex chain tests
#[tokio::test]
async fn complex_condition_chain() {
    let command = execute!(
        if ExecuteConditionCategory::SCORE(
            ExecuteScoreCommand::MATCHES {
                target: TargetSelector::Current(None),
                objective: "points".to_string(),
                range: "10..".to_string(),
            }
        ) =>
        as TargetSelector::Current(None) =>
        at TargetSelector::All(None) =>
        facing ExecuteFacing::TARGETS(TargetSelector::AllEntities(None)) =>
        anchored ExecuteAnchor::Eyes =>
        positioned ExecutePositioned::HEIGHTMAP(HeightMap::WorldSurface) =>
        store ExecuteStoreAction::SUCCESS,
        ExecuteStoreCommand::SCORE {
            target: TargetSelector::Current(None),
            objective: "success_count".to_string(),
        } =>
        run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}

#[tokio::test]
async fn nested_conditions() {
    let command = execute!(
        if ExecuteConditionCategory::LOADED(Location::new(0.0, 0.0, 0.0)) =>
        unless ExecuteConditionCategory::ENTITY(TargetSelector::Current(None)) =>
        run Kill::new(TargetSelector::Current(None))
    );
    assert_validation!(command, true);
}
