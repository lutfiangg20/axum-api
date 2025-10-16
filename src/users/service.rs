use crate::users::repository::UserRepository;

pub struct UserService {
    pub user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub async fn get_all_users(&self) -> Result<Vec<crate::users::model::User>, sqlx::Error> {
        let users = self.user_repository.find_all().await?;
        Ok(users)
    }

    pub async fn create_user(
        &self,
        create: crate::users::model::CreateUser,
    ) -> Result<u64, sqlx::Error> {
        let result = self.user_repository.save(create).await?;
        Ok(result)
    }
}
