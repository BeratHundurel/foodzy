use std::sync::Arc;

use crate::domains::{
    auth::AuthServiceTrait, product::ProductServiceTrait, user::UserServiceTrait,
};

use super::config::Config;

/// AppState is a struct that holds the application-wide shared state.
/// It is passed to request handlers via Axum's extension mechanism.
#[derive(Clone)]
pub struct AppState {
    /// Global application configuration.
    pub config: Config,
    /// Service handling authentication-related logic.
    pub auth_service: Arc<dyn AuthServiceTrait>,
    /// Service handling user-related logic.
    pub user_service: Arc<dyn UserServiceTrait>,
    pub product_service: Arc<dyn ProductServiceTrait>,
}

impl AppState {
    /// Creates a new instance of AppState with the provided dependencies.
    pub fn new(
        config: Config,
        auth_service: Arc<dyn AuthServiceTrait>,
        user_service: Arc<dyn UserServiceTrait>,
        product_service: Arc<dyn ProductServiceTrait>,
    ) -> Self {
        Self {
            config,
            auth_service,
            user_service,
            product_service,
        }
    }
}
