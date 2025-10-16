use crate::products::{
    model::{CreateProduct, Product},
    repository::ProductRepository,
};

pub struct ProductsService {
    pub product_repository: ProductRepository,
}

impl ProductsService {
    pub fn new(product_repository: ProductRepository) -> Self {
        Self { product_repository }
    }

    pub async fn get_all_products(&self) -> Result<Vec<Product>, sqlx::Error> {
        let products = self.product_repository.find_all().await?;
        Ok(products)
    }

    pub async fn create_product(&self, create: CreateProduct) -> Result<u64, sqlx::Error> {
        let result = self.product_repository.save(create).await?;
        Ok(result)
    }
}
