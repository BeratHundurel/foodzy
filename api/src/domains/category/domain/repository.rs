use super::model::Category;

use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn find_all(&self, pool: PgPool) -> Result<Vec<Category>, sqlx::Error>;

    async fn find_by_id(&self, pool: PgPool, id: i32) -> Result<Option<Category>, sqlx::Error>;
}
