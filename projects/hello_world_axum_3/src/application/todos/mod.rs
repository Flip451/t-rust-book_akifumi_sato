pub mod todo_create_application_service;
pub mod todo_get_application_service;
pub mod todo_get_all_aplication_service;
pub mod todo_update_application_service;
pub mod todo_delete_application_service;
pub mod todo_data;
pub mod todo_application_error;


use self::todo_application_error::TodoApplicationError;

pub type Result<T> = anyhow::Result<T, TodoApplicationError>;