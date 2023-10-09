pub mod label_create_application_service;
pub mod label_get_application_service;
pub mod label_get_all_aplication_service;
pub mod label_update_application_service;
pub mod label_delete_application_service;
pub mod label_data;
pub mod label_application_error;

use anyhow::Result as AnyhowResult;

use self::label_application_error::LabelApplicationError;

pub type Result<T> = AnyhowResult<T, LabelApplicationError>;