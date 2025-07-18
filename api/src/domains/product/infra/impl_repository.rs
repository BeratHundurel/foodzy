use std::str::FromStr;

use crate::domains::product::domain::{
    model::{Product, ProductWithCategory},
    repository::ProductRepository,
};
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use sqlx::PgPool;

pub struct ProductRepo;

#[async_trait]
impl ProductRepository for ProductRepo {
    async fn find_all(&self, pool: PgPool) -> Result<Vec<Product>, sqlx::Error> {
        let products = sqlx::query_as!(
            Product,
            r#"
            SELECT id, name, description, price, is_best_seller, is_deal_of_the_day, discount, category_id
            FROM products
            "#
        )
        .fetch_all(&pool)
        .await?;
        Ok(products)
    }

    async fn find_by_id(&self, pool: PgPool, id: i32) -> Result<Option<Product>, sqlx::Error> {
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT id, name, description, price, is_best_seller, is_deal_of_the_day, discount, category_id
            FROM products
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&pool)
        .await?;
        Ok(product)
    }

    async fn find_by_category_id(
        &self,
        pool: PgPool,
        category_id: i32,
    ) -> Result<Vec<ProductWithCategory>, sqlx::Error> {
        let products = sqlx::query_as!(
            ProductWithCategory,
            r#"
            SELECT p.id, p.name, p.description, p.price, p.is_best_seller, p.is_deal_of_the_day, 
                   p.discount, p.category_id, c.name as category_name
            FROM products p
            INNER JOIN categories c ON p.category_id = c.id
            WHERE p.category_id = $1
            "#,
            category_id
        )
        .fetch_all(&pool)
        .await?;
        Ok(products)
    }

    async fn find_best_sellers(
        &self,
        pool: PgPool,
        limit: i64,
    ) -> Result<Vec<Product>, sqlx::Error> {
        let products = sqlx::query_as!(
            Product,
            r#"
            SELECT id, name, description, price, is_best_seller, is_deal_of_the_day, discount, category_id
            FROM products
            WHERE is_best_seller = true
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(&pool)
        .await?;
        Ok(products)
    }

    async fn find_deals_of_the_day(
        &self,
        pool: PgPool,
        limit: i64,
    ) -> Result<Vec<Product>, sqlx::Error> {
        let products = sqlx::query_as!(
            Product,
            r#"
            SELECT id, name, description, price, is_best_seller, is_deal_of_the_day, discount, category_id
            FROM products
            WHERE is_deal_of_the_day = true
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(&pool)
        .await?;
        Ok(products)
    }

    async fn find_by_price_range(
        &self,
        pool: PgPool,
        min_price: BigDecimal,
        max_price: BigDecimal,
    ) -> Result<Vec<Product>, sqlx::Error> {
        let products = sqlx::query_as!(
            Product,
            r#"
            SELECT id, name, description, price, is_best_seller, is_deal_of_the_day, discount, category_id
            FROM products
            WHERE price BETWEEN $1 AND $2
            "#,
            min_price,
            max_price
        )
        .fetch_all(&pool)
        .await?;
        Ok(products)
    }

    async fn find_by_filter(
        &self,
        pool: PgPool,
        category: Option<String>,
        is_best_seller: Option<bool>,
        is_deal_of_the_day: Option<bool>,
        min_price: Option<String>,
        max_price: Option<String>,
    ) -> Result<Vec<ProductWithCategory>, sqlx::Error> {
        let mut query_builder = sqlx::QueryBuilder::new(
            "SELECT p.id, p.name, p.description, p.price, p.is_best_seller, p.is_deal_of_the_day, 
                    p.discount, p.category_id, c.name as category_name
             FROM products p
             INNER JOIN categories c ON p.category_id = c.id
             WHERE 1=1",
        );

        if let Some(cat) = &category {
            query_builder.push(" AND c.name ILIKE ");
            query_builder.push_bind(format!("%{}%", cat));
        }
        if let Some(best_seller) = is_best_seller {
            query_builder.push(" AND p.is_best_seller = ");
            query_builder.push_bind(best_seller);
        }
        if let Some(deal_of_the_day) = is_deal_of_the_day {
            query_builder.push(" AND p.is_deal_of_the_day = ");
            query_builder.push_bind(deal_of_the_day);
        }
        if let Some(min) = &min_price {
            // Convert min_price to BigDecimal, only add filter if conversion succeeds
            if let Ok(min_val) = BigDecimal::from_str(min) {
                query_builder.push(" AND p.price >= ");
                query_builder.push_bind(min_val);
            }
        }
        if let Some(max) = &max_price {
            if let Ok(max_val) = BigDecimal::from_str(max) {
                query_builder.push(" AND p.price <= ");
                query_builder.push_bind(max_val);
            }
        }

        let products = query_builder
            .build_query_as::<ProductWithCategory>()
            .fetch_all(&pool)
            .await?;

        Ok(products)
    }
}
