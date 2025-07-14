use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domains::product::domain::model::Product;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductDto {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: String,
    pub isbestseller: bool,
    pub isdealoftheday: bool,
    pub discount: String,
    pub category_id: i32,
}

impl From<Product> for ProductDto {
    fn from(user: Product) -> Self {
        Self {
            id: user.id,
            name: user.name,
            description: user.description,
            price: user.price.to_string(),
            isbestseller: user.isbestseller,
            isdealoftheday: user.isdealoftheday,
            discount: user.discount.to_string(),
            category_id: user.category_id,
        }
    }
}
