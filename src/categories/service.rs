use crate::categories::{
    model::{CreateCategory, Product},
    repository::CategoryRepository,
};

pub struct CategoryService {
    pub category_repository: CategoryRepository,
}

impl CategoryService {
    pub fn new(category_repository: CategoryRepository) -> Self {
        Self {
            category_repository,
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<Product>, sqlx::Error> {
        let categories = self.category_repository.find_all().await?;
        Ok(categories)
    }

    pub async fn create_user(&self, create: CreateCategory) -> Result<u64, sqlx::Error> {
        let result = self.category_repository.save(create).await?;
        Ok(result)
    }
}
