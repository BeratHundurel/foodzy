use crate::domains::product::domain::model::ProductWithCategory;

use super::model::Product;

use async_trait::async_trait;
use bigdecimal::BigDecimal;
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

    async fn find_by_price_range(
        &self,
        pool: PgPool,
        min_price: BigDecimal,
        max_price: BigDecimal,
    ) -> Result<Vec<Product>, sqlx::Error>;

    async fn find_by_filter(
        &self,
        pool: PgPool,
        category: Option<String>,
        is_best_seller: Option<bool>,
        is_deal_of_the_day: Option<bool>,
        min_price: Option<String>,
        max_price: Option<String>,
    ) -> Result<Vec<ProductWithCategory>, sqlx::Error>;

}
