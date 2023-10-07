mod user_create_application_service;
mod user_get_application_service;
mod user_get_all_aplication_service;
mod user_update_application_service;
mod user_delete_application_service;
mod user_data;
mod user_application_error;

use anyhow::Result as AnyhowResult;

use self::user_application_error::UserApplicationError;

pub type Result<T> = AnyhowResult<T, UserApplicationError>;