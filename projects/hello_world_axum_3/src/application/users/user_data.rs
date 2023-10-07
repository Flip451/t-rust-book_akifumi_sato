use serde::Serialize;
use uuid::Uuid;

use crate::domain::{models::users::User, value_object::ValueObject};

#[derive(Serialize, PartialEq, Debug)]
pub struct UserData {
    pub user_id: Uuid,
    pub user_name: String,
}

impl UserData {
    pub fn new(user: User) -> Self {
        let user_id = user.user_id().clone().into_value();
        let User { user_name, .. } = user;
        Self {
            user_id,
            user_name: user_name.into_value(),
        }
    }
}
