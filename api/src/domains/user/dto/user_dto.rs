use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::domains::user::domain::model::User;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SearchUserDto {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateUserMultipartDto {
    #[validate(length(max = 64, message = "Username cannot exceed 64 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateUserDto {
    #[validate(length(max = 64, message = "Username cannot exceed 64 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}
