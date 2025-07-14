use crate::{
    common::error::AppError,
    domains::product::{
        domain::{repository::ProductRepository, service::ProductServiceTrait},
        dto::product_dto::ProductDto,
        infra::impl_repository::ProductRepo,
    },
};
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

/// Service struct for handling user-related operations
/// such as creating, updating, deleting, and fetching users.
/// It uses a repository pattern to abstract the data access layer.
#[derive(Clone)]
pub struct ProductService {
    pub pool: PgPool,
    pub repo: Arc<dyn ProductRepository + Send + Sync>,
}

#[async_trait]
impl ProductServiceTrait for ProductService {
    /// constructor for the service.
    fn create_service(pool: PgPool) -> Arc<dyn ProductServiceTrait> {
        Arc::new(Self {
            pool,
            repo: Arc::new(ProductRepo {}),
        })
    }

    async fn get_product_by_id(&self, id: i32) -> Result<ProductDto, AppError> {
        match self.repo.find_by_id(self.pool.clone(), id).await {
            Ok(Some(product)) => Ok(ProductDto::from(product)),
            Ok(None) => Err(AppError::NotFound("Product not found".into())),
            Err(err) => {
                tracing::error!("Error retrieving product: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    async fn get_products(&self) -> Result<Vec<ProductDto>, AppError> {
        match self.repo.find_all(self.pool.clone()).await {
            Ok(products) => {
                let product_dtos: Vec<ProductDto> = products.into_iter().map(Into::into).collect();
                Ok(product_dtos)
            }
            Err(err) => {
                tracing::error!("Error fetching products: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    async fn get_products_by_category_id(
        &self,
        category_id: i32,
    ) -> Result<Vec<ProductDto>, AppError> {
        match self
            .repo
            .find_by_category_id(self.pool.clone(), category_id)
            .await
        {
            Ok(products) => {
                let product_dtos: Vec<ProductDto> = products.into_iter().map(Into::into).collect();
                Ok(product_dtos)
            }
            Err(err) => {
                tracing::error!("Error fetching products by category: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }
}
