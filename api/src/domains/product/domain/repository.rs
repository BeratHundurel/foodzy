use crate::domains::product::domain::model::ProductWithCategory;

use super::model::Product;

use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn find_all(&self, pool: PgPool) -> Result<Vec<Product>, sqlx::Error>;

    async fn find_by_id(&self, pool: PgPool, id: i32) -> Result<Option<Product>, sqlx::Error>;

    async fn find_by_category_id(
        &self,
        pool: PgPool,
        category_id: i32,
    ) -> Result<Vec<ProductWithCategory>, sqlx::Error>;

    async fn find_best_sellers(
        &self,
        pool: PgPool,
        limit: i64,
    ) -> Result<Vec<Product>, sqlx::Error>;

    async fn find_deals_of_the_day(
        &self,
        pool: PgPool,
        limit: i64,
    ) -> Result<Vec<Product>, sqlx::Error>;
}
