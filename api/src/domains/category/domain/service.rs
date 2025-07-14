use crate::{common::error::AppError, domains::category::dto::category_dto::CategoryDto};

use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait]
pub trait CategoryServiceTrait: Send + Sync {
    /// constructor for the service.
    fn create_service(pool: PgPool) -> Arc<dyn CategoryServiceTrait>
    where
        Self: Sized;

    async fn get_category_by_id(&self, id: i32) -> Result<CategoryDto, AppError>;

    async fn get_categories(&self) -> Result<Vec<CategoryDto>, AppError>;
}
