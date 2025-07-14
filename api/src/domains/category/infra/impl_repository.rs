use crate::domains::category::domain::{model::Category, repository::CategoryRepository};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct CategoryRepo;

const FIND_ALL_CATEGORIES_QUERY: &str = r#"
    SELECT *
    FROM categories
"#;

const FIND_CATEGORY_BY_ID_QUERY: &str = r#"
    SELECT *
    FROM categories
    WHERE id = $1
"#;

#[async_trait]
impl CategoryRepository for CategoryRepo {
    async fn find_all(&self, pool: PgPool) -> Result<Vec<Category>, sqlx::Error> {
        let categories = sqlx::query_as::<_, Category>(FIND_ALL_CATEGORIES_QUERY)
            .fetch_all(&pool)
            .await?;
        Ok(categories)
    }

    async fn find_by_id(&self, pool: PgPool, id: i32) -> Result<Option<Category>, sqlx::Error> {
        let category = sqlx::query_as::<_, Category>(FIND_CATEGORY_BY_ID_QUERY)
            .bind(id)
            .fetch_optional(&pool)
            .await?;
        Ok(category)
    }
}
