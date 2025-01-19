use crate::{assert_validation, enchantments::Enchantments, prelude::MinecraftEntity, target_filter, types::TargetSelector};

#[tokio::test]
async fn basic_enchant() {
    let command =
        crate::commands::Enchant::new(Enchantments::Sharpness(3), TargetSelector::Current(None));

    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn filter_basic_enchant() {
    let command = crate::commands::Enchant::new(
        Enchantments::DepthStrider(1),
        TargetSelector::All(Some(target_filter!(@MinecraftEntity, distance = "..300", team = "Red"))),
    );

    assert_validation!(command.to_string(), true);
}
