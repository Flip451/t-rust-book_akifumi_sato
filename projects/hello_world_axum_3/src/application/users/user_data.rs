use uuid::Uuid;

use crate::domain::{models::users::User, value_object::ValueObject};

pub struct UserData(User);

impl UserData {
    pub fn new(source: User) -> Self {
        Self(source)
    }

    pub fn get_user_id(&self) -> &Uuid {
        let UserData(user) = self;
        user.user_id().value()
    }

    pub fn get_user_name(&self) -> &String {
        let UserData(user) = self;
        user.user_name.value()
    }
}