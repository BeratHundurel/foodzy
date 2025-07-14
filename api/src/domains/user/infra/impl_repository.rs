use crate::domains::user::{
    domain::{model::User, repository::UserRepository},
    dto::user_dto::{CreateUserMultipartDto, SearchUserDto, UpdateUserDto},
};
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, QueryBuilder, Transaction};

pub struct UserRepo;

const FIND_USER_QUERY: &str = r#"
    SELECT
        u.id,
        u.username,
        u.email
    FROM users u
    WHERE 1=1
"#;

const FIND_USER_INFO_QUERY: &str = r#"
    SELECT
        u.id,
        u.username,
        u.email
    FROM users u
    WHERE u.id = $1
"#;

#[async_trait]
impl UserRepository for UserRepo {
    async fn find_all(&self, pool: PgPool) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(FIND_USER_QUERY)
            .fetch_all(&pool)
            .await?;
        Ok(users)
    }

    async fn find_list(
        &self,
        pool: PgPool,
        search_user_dto: SearchUserDto,
    ) -> Result<Vec<User>, sqlx::Error> {
        let mut builder = QueryBuilder::<Postgres>::new(FIND_USER_QUERY);

        if let Some(id) = search_user_dto.id.filter(|id| *id != 0) {
            builder.push(" AND u.id = ");
            builder.push_bind(id);
        }

        if let Some(s) = search_user_dto
            .username
            .as_deref()
            .filter(|s| !s.trim().is_empty())
        {
            builder.push(" AND u.username ILIKE ");
            builder.push_bind(format!("%{}%", s));
        }

        if let Some(e) = search_user_dto
            .email
            .as_deref()
            .filter(|e| !e.trim().is_empty())
        {
            builder.push(" AND u.email ILIKE ");
            builder.push_bind(format!("%{}%", e));
        }

        let query = builder.build_query_as::<User>();
        let users = query.fetch_all(&pool).await?;
        Ok(users)
    }

    async fn find_by_id(&self, pool: PgPool, id: String) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(FIND_USER_INFO_QUERY)
            .bind(id.parse::<i32>().unwrap_or_default())
            .fetch_optional(&pool)
            .await?;
        Ok(user)
    }

    async fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user: CreateUserMultipartDto,
    ) -> Result<String, sqlx::Error> {
        let inserted = sqlx::query!(
            r#"
                INSERT INTO users (username, email)
                VALUES ($1, $2)
                RETURNING id
            "#,
            user.username,
            Some(user.email)
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(inserted.id.to_string())
    }

    async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: String,
        user: UpdateUserDto,
    ) -> Result<Option<User>, sqlx::Error> {
        let user_id = id.parse::<i32>().unwrap_or_default();

        let existing = sqlx::query_as::<_, User>(FIND_USER_INFO_QUERY)
            .bind(user_id)
            .fetch_optional(&mut **tx)
            .await?;

        if existing.is_some() {
            sqlx::query!(
                r#"
                    UPDATE users
                    SET username = $1,
                        email = $2
                    WHERE id = $3
                "#,
                user.username,
                Some(user.email),
                user_id
            )
            .execute(&mut **tx)
            .await?;

            let updated_user = sqlx::query_as::<_, User>(FIND_USER_INFO_QUERY)
                .bind(user_id)
                .fetch_one(&mut **tx)
                .await?;

            return Ok(Some(updated_user));
        }

        Ok(None)
    }

    async fn delete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: String,
    ) -> Result<bool, sqlx::Error> {
        let user_id = id.parse::<i32>().unwrap_or_default();
        let res = sqlx::query!(
            r#"DELETE FROM users WHERE id = $1"#,
            user_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(res.rows_affected() > 0)
    }
}