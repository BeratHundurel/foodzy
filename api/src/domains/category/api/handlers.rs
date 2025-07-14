use crate::{
    common::{app_state::AppState, dto::RestApiResponse, error::AppError},
    domains::category::dto::category_dto::CategoryDto,
};

use axum::{extract::State, response::IntoResponse};

#[utoipa::path(
    get,
    path = "/category/{id}",
    responses((status = 200, description = "Get category by ID", body = CategoryDto)),
    tag = "Categories"
)]
pub async fn get_category_by_id(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let id: i32 = id.parse().map_err(|_| AppError::InternalError)?;
    let product = state.category_service.get_category_by_id(id).await?;
    Ok(RestApiResponse::success(product))
}

#[utoipa::path(
    get,
    path = "/category",
    responses((status = 200, description = "List all categories", body = [CategoryDto])),
    tag = "Categories"
)]
pub async fn get_categories(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let products = state.category_service.get_categories().await?;
    Ok(RestApiResponse::success(products))
}
