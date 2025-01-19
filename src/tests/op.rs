use crate::{assert_validation, prelude::op::OP, types::OPAction};



#[tokio::test]
async fn grant_op() {
    let command = OP::new(OPAction::GRANT("Toast".to_string()));
    assert_validation!(command, true);
}

#[tokio::test]
async fn revoke_op() {
    let command = OP::new(OPAction::REVOKE("Toast".to_string()));
    assert_validation!(command, true);
}

