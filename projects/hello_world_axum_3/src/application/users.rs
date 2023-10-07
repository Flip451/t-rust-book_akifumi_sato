pub mod user_create_application_service;
pub mod user_get_application_service;
pub mod user_get_all_aplication_service;
pub mod user_update_application_service;
pub mod user_delete_application_service;
pub mod user_data;
pub mod user_application_error;

use anyhow::Result as AnyhowResult;

use self::user_application_error::UserApplicationError;

pub type Result<T> = AnyhowResult<T, UserApplicationError>;