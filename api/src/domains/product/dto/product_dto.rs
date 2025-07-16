use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domains::product::domain::model::{Product, ProductWithCategory};

#[derive(Deserialize)]
pub struct BestSellerQuery {
    pub limit: Option<i64>,
}

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
    pub category_name: Option<String>,
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
            category_name: None,
        }
    }
}

impl From<ProductWithCategory> for ProductDto {
    fn from(product: ProductWithCategory) -> Self {
        Self {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price.to_string(),
            isbestseller: product.isbestseller,
            isdealoftheday: product.isdealoftheday,
            discount: product.discount.to_string(),
            category_id: product.category_id,
            category_name: Some(product.category_name),
        }
    }
}
