use crate::{
    common::{app_state::AppState, dto::RestApiResponse, error::AppError},
    domains::product::dto::product_dto::ProductDto,
};

use axum::{extract::State, response::IntoResponse};

#[utoipa::path(
    get,
    path = "/product/{id}",
    responses((status = 200, description = "Get product by ID", body = ProductDto)),
    tag = "Products"
)]
pub async fn get_product_by_id(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let id: i32 = id.parse().map_err(|_| AppError::InternalError)?;
    let product = state.product_service.get_product_by_id(id).await?;
    Ok(RestApiResponse::success(product))
}

#[utoipa::path(
    get,
    path = "/product",
    responses((status = 200, description = "List all product", body = [ProductDto])),
    tag = "Products"
)]
pub async fn get_products(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let products = state.product_service.get_products().await?;
    Ok(RestApiResponse::success(products))
}
