use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domains::product::domain::model::{Product, ProductWithCategory};

#[derive(Deserialize, ToSchema)]
pub struct BestSellerQuery {
    #[schema(example = 10)]
    pub limit: Option<i64>,
}

#[derive(Deserialize, ToSchema)]
pub struct PriceRangeQuery {
    #[schema(example = "10.5")]
    pub min_price: String,
    #[schema(example = "50.0")]
    pub max_price: String,
}

#[derive(Deserialize, ToSchema)]
pub struct FilterQuery {
    #[schema(example = "Breakfast")]
    pub category: Option<String>,
    #[schema(example = "true")]
    pub is_best_seller: Option<bool>,
    #[schema(example = "true")]
    pub is_deal_of_the_day: Option<bool>,
    #[schema(example = "10.0")]
    pub min_price: Option<String>,
    #[schema(example = "100.0")]
    pub max_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductDto {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: String,
    pub is_best_seller: bool,
    pub is_deal_of_the_day: bool,
    pub discount: String,
    pub category_id: i32,
    pub category_name: Option<String>,
}

impl From<Product> for ProductDto {
    fn from(product: Product) -> Self {
        Self {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price.to_string(),
            is_best_seller: product.is_best_seller,
            is_deal_of_the_day: product.is_deal_of_the_day,
            discount: product.discount.to_string(),
            category_id: product.category_id,
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
            is_best_seller: product.is_best_seller,
            is_deal_of_the_day: product.is_deal_of_the_day,
            discount: product.discount.to_string(),
            category_id: product.category_id,
            category_name: Some(product.category_name),
        }
    }
}
