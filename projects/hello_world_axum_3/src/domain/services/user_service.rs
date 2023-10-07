use crate::{
    domain::models::users::User,
    infra::repository::users::{IUserRepository, Result},
};

pub struct UserService<T: IUserRepository> {
    user_repository: T,
}

impl<T: IUserRepository> UserService<T> {
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn is_duplicated(&self, user: &User) -> Result<bool> {
        let user_name = &user.user_name;
        let search_result = self.user_repository.find_by_name(user_name).await?;
        match search_result {
            Some(user_found) => Ok(!(&user_found == user)),
            None => Ok(false),
        }
    }
}
