use super::handlers::*;
use crate::{common::app_state::AppState, domains::product::dto::product_dto::ProductDto};

use axum::{routing::get, Router};

use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_product_by_id,
        get_products,
        get_products_by_category_id,
        get_best_sellers,
        get_deals_of_the_day
    ),
    components(schemas(ProductDto)),
    tags(
        (name = "Products", description = "Product management endpoints")
    ),
    security(
        ("bearer_auth" = [])
    ),
    modifiers(&ProductApiDoc)
)]
/// This struct is used to generate OpenAPI documentation for the user routes.
pub struct ProductApiDoc;

impl utoipa::Modify for ProductApiDoc {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some("Input your `<your‑jwt>`"))
                    .build(),
            ),
        )
    }
}

pub fn product_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_products))
        .route("/{id}", get(get_product_by_id))
        .route("/category/{category_id}", get(get_products_by_category_id))
        .route("/best-sellers", get(get_best_sellers))
        .route("/deal-of-the-day", get(get_deals_of_the_day))
}
