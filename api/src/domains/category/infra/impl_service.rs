use crate::{
    common::error::AppError,
    domains::category::{
        domain::{repository::CategoryRepository, service::CategoryServiceTrait},
        dto::category_dto::CategoryDto,
        infra::impl_repository::CategoryRepo,
    },
};
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

/// Service struct for handling user-related operations
/// such as creating, updating, deleting, and fetching users.
/// It uses a repository pattern to abstract the data access layer.
#[derive(Clone)]
pub struct CategoryService {
    pub pool: PgPool,
    pub repo: Arc<dyn CategoryRepository + Send + Sync>,
}

#[async_trait]
impl CategoryServiceTrait for CategoryService {
    /// constructor for the service.
    fn create_service(pool: PgPool) -> Arc<dyn CategoryServiceTrait> {
        Arc::new(Self {
            pool,
            repo: Arc::new(CategoryRepo {}),
        })
    }

    async fn get_category_by_id(&self, id: i32) -> Result<CategoryDto, AppError> {
        match self.repo.find_by_id(self.pool.clone(), id).await {
            Ok(Some(category)) => Ok(CategoryDto::from(category)),
            Ok(None) => Err(AppError::NotFound("Category not found".into())),
            Err(err) => {
                tracing::error!("Error retrieving category: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    async fn get_categories(&self) -> Result<Vec<CategoryDto>, AppError> {
        match self.repo.find_all(self.pool.clone()).await {
            Ok(categories) => {
                let category_dtos: Vec<CategoryDto> =
                    categories.into_iter().map(Into::into).collect();
                Ok(category_dtos)
            }
            Err(err) => {
                tracing::error!("Error fetching products: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }
}
