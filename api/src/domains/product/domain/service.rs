use crate::{common::error::AppError, domains::product::dto::product_dto::ProductDto};

use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait]
pub trait ProductServiceTrait: Send + Sync {
    /// constructor for the service.
    fn create_service(pool: PgPool) -> Arc<dyn ProductServiceTrait>
    where
        Self: Sized;

    async fn get_product_by_id(&self, id: i32) -> Result<ProductDto, AppError>;

    async fn get_products(&self) -> Result<Vec<ProductDto>, AppError>;
}
