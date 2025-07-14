use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct AuthUserDto {
    pub user_id: i32,
    pub password: String,
}
