use crate::{assert_validation, prelude::difficulty::Difficulty};


#[tokio::test]
async fn peaceful() {
    let command = Difficulty::new(crate::types::world::Difficulty::Peaceful);
    assert_validation!(command, true);
}

#[tokio::test]
async fn easy() {
    let command = Difficulty::new(crate::types::world::Difficulty::Easy);
    assert_validation!(command, true);
}

#[tokio::test]
async fn normal() {
    let command = Difficulty::new(crate::types::world::Difficulty::Normal);
    assert_validation!(command, true);
}

#[tokio::test]
async fn hard() {
    let command = Difficulty::new(crate::types::world::Difficulty::Hard);
    assert_validation!(command, true);
}