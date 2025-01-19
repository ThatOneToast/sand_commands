use crate::{assert_validation, prelude::Save};



#[tokio::test]
async fn save_all() {
    let command = Save::new(crate::prelude::SaveAction::ALL);
    assert_validation!(command, true)
}

#[tokio::test]
async fn save_on() {
    let command = Save::new(crate::prelude::SaveAction::ON);
    assert_validation!(command, true)
}

#[tokio::test]
async fn save_off() {
    let command = Save::new(crate::prelude::SaveAction::OFF);
    assert_validation!(command, true)
}