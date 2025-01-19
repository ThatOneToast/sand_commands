use crate::{assert_validation, prelude::Setblock, types::Location};



#[tokio::test]
async fn set_snow() {
    let command = Setblock {
        at: Location {
            x: 10.0,
            y: 67.0,
            z: 44.0
        },
        with: "minecraft:snow".to_string()
    };
    assert_validation!(command, true)
}