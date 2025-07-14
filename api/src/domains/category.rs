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
    pub mod category_dto;
}

mod infra {
    mod impl_repository;
    pub mod impl_service;
}

pub use api::routes::{category_routes, CategoryApiDoc};
pub use domain::service::CategoryServiceTrait;
pub use infra::impl_service::CategoryService;
