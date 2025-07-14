use sqlx::prelude::FromRow;
#[derive(Debug, Clone, FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
}
