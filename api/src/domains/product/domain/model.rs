use bigdecimal::BigDecimal;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub is_best_seller: bool,
    pub is_deal_of_the_day: bool,
    pub discount: BigDecimal,
    pub category_id: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct ProductWithCategory {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub is_best_seller: bool,
    pub is_deal_of_the_day: bool,
    pub discount: BigDecimal,
    pub category_id: i32,
    pub category_name: String,
}
