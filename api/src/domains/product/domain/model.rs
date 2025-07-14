use bigdecimal::BigDecimal;
use sqlx::prelude::FromRow;
#[derive(Debug, Clone, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub isbestseller: bool,
    pub isdealoftheday: bool,
    pub discount: BigDecimal,
    pub category_id: i32,
}
