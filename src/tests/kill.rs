use crate::{assert_validation, prelude::{Kill, MinecraftEntity}, target_filter, types::TargetSelector};



#[tokio::test]
async fn kill_self() {
    let command = Kill::new_self();
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn kill_cows() {
    let command = Kill::new(TargetSelector::AllEntities(Some(target_filter!(@MinecraftEntity, entity = MinecraftEntity::Cow))));
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn kill_things_with() {
    let command = Kill::new(TargetSelector::AllEntities(
        Some(
            target_filter!(@MinecraftEntity, name = "John", level = "..30")
        )
    ));
    assert_validation!(command.to_string(), true);
}