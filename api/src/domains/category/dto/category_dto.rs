use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domains::category::domain::model::Category;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryDto {
    pub id: i32,
    pub name: String,
}

impl From<Category> for CategoryDto {
    fn from(category: Category) -> Self {
        Self {
            id: category.id,
            name: category.name,
        }
    }
}
