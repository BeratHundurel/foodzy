use crate::{common::error::AppError, domains::product::dto::product_dto::ProductDto};

use async_trait::async_trait;
use bigdecimal::BigDecimal;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait]
pub trait ProductServiceTrait: Send + Sync {
    fn create_service(pool: PgPool) -> Arc<dyn ProductServiceTrait>
    where
        Self: Sized;

    async fn get_product_by_id(&self, id: i32) -> Result<ProductDto, AppError>;

    async fn get_products(&self) -> Result<Vec<ProductDto>, AppError>;

    async fn get_products_by_category_id(
        &self,
        category_id: i32,
    ) -> Result<Vec<ProductDto>, AppError>;

    async fn get_best_sellers(&self, limit: i64) -> Result<Vec<ProductDto>, AppError>;

    async fn get_deals_of_the_day(&self, limit: i64) -> Result<Vec<ProductDto>, AppError>;

    async fn get_products_by_price_range(
        &self,
        min_price: BigDecimal,
        max_price: BigDecimal,
    ) -> Result<Vec<ProductDto>, AppError>;
}
