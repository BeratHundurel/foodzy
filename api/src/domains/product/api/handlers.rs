use crate::{
    common::{app_state::AppState, dto::RestApiResponse, error::AppError},
    domains::product::dto::product_dto::{BestSellerQuery, PriceRangeQuery, ProductDto},
};

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use bigdecimal::BigDecimal;

#[utoipa::path(
    get,
    path = "/product/{id}",
    responses((status = 200, description = "Get product by ID", body = ProductDto)),
    tag = "Products"
)]
pub async fn get_product_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
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

#[utoipa::path(
    get,
    path = "/product/category/{category_id}",
    responses((status = 200, description = "Get products by category ID", body = [ProductDto])),
    tag = "Products"
)]
pub async fn get_products_by_category_id(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let category_id: i32 = category_id.parse().map_err(|_| AppError::InternalError)?;
    let products = state
        .product_service
        .get_products_by_category_id(category_id)
        .await?;
    Ok(RestApiResponse::success(products))
}

#[utoipa::path(
    get,
    path = "/product/best-sellers",
    responses((status = 200, description = "Get best-selling products", body = [ProductDto])),
    tag = "Products"
)]
pub async fn get_best_sellers(
    State(state): State<AppState>,
    params: Query<BestSellerQuery>,
) -> Result<impl IntoResponse, AppError> {
    let limit = params.limit.unwrap_or(10);
    let products = state.product_service.get_best_sellers(limit).await?;
    Ok(RestApiResponse::success(products))
}

#[utoipa::path(
    get,
    path = "/product/deal-of-the-day",
    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of products to return")
    ),
    responses(
        (status = 200, description = "Get deals of the day", body = [ProductDto])
    ),
    tag = "Products"
)]
pub async fn get_deals_of_the_day(
    State(state): State<AppState>,
    params: Query<BestSellerQuery>,
) -> Result<impl IntoResponse, AppError> {
    let limit = params.limit.unwrap_or(10);
    let products = state.product_service.get_deals_of_the_day(limit).await?;
    Ok(RestApiResponse::success(products))
}

#[utoipa::path(
    get,
    path = "/product/price-range",
    params(
        ("min_price" = String, Query, description = "Minimum price"),
        ("max_price" = String, Query, description = "Maximum price")
    ),
    responses((status = 200, description = "Get products by price range", body = [ProductDto])),
    tag = "Products"
)]
pub async fn get_products_by_price_range(
    State(state): State<AppState>,
    Query(query): Query<PriceRangeQuery>,
) -> Result<impl IntoResponse, AppError> {
    let min_price: BigDecimal = query
        .min_price
        .parse()
        .map_err(|_| AppError::InternalError)?;
    let max_price: BigDecimal = query
        .max_price
        .parse()
        .map_err(|_| AppError::InternalError)?;

    let products = state
        .product_service
        .get_products_by_price_range(min_price, max_price)
        .await?;

    Ok(RestApiResponse::success(products))
}
