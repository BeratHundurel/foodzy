use super::handlers::*;
use crate::{common::app_state::AppState, domains::category::dto::category_dto::CategoryDto};

use axum::{routing::get, Router};

use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_category_by_id,
        get_categories,
    ),
    components(schemas(CategoryDto)),
    tags(
        (name = "Categories", description = "Category management endpoints")
    ),
    security(
        ("bearer_auth" = [])
    ),
    modifiers(&CategoryApiDoc)
)]
/// This struct is used to generate OpenAPI documentation for the user routes.
pub struct CategoryApiDoc;

impl utoipa::Modify for CategoryApiDoc {
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

pub fn category_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_categories))
        .route("/{id}", get(get_category_by_id))
}
