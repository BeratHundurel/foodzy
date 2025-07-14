mod api {
    mod handlers;
    pub mod routes;
}

mod domain {
    pub mod model;
    pub mod repository;
    pub mod service;
}

pub mod dto {
    pub mod product_dto;
}

mod infra {
    mod impl_repository;
    pub mod impl_service;
}

pub use api::routes::{product_routes, ProductApiDoc};
pub use domain::service::ProductServiceTrait;
pub use infra::impl_service::ProductService;