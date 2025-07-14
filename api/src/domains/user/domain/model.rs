use sqlx::FromRow;
/// Domain model representing a user in the application.
#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
}
