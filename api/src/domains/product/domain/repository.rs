use super::model::Product;

use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn find_all(&self, pool: PgPool) -> Result<Vec<Product>, sqlx::Error>;

    async fn find_by_id(&self, pool: PgPool, id: i32) -> Result<Option<Product>, sqlx::Error>;
}
