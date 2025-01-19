use crate::{assert_validation, prelude::fill::Fill, types::Location};



#[tokio::test]
async fn hundred_wool() {
    let command = Fill {
        from: Location {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        to: Location {
            x: 10.0,
            y: 0.0,
            z: 10.0
        },
        with: "minecraft:white_wool".to_string()
    };
    assert_validation!(command, true)
}
