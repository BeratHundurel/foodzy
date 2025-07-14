use crate::domains::product::domain::{model::Product, repository::ProductRepository};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct ProductRepo;

const FIND_ALL_PRODUCTS_QUERY: &str = r#"
    SELECT *
    FROM products
"#;

const FIND_PRODUCT_BY_ID_QUERY: &str = r#"
    SELECT *
    FROM products
    WHERE id = $1
"#;

#[async_trait]
impl ProductRepository for ProductRepo {
    async fn find_all(&self, pool: PgPool) -> Result<Vec<Product>, sqlx::Error> {
        let products = sqlx::query_as::<_, Product>(FIND_ALL_PRODUCTS_QUERY)
            .fetch_all(&pool)
            .await?;
        Ok(products)
    }

    async fn find_by_id(&self, pool: PgPool, id: i32) -> Result<Option<Product>, sqlx::Error> {
        let product = sqlx::query_as::<_, Product>(FIND_PRODUCT_BY_ID_QUERY)
            .bind(id)
            .fetch_optional(&pool)
            .await?;
        Ok(product)
    }
}
