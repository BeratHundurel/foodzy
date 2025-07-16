use crate::{
    common::error::AppError,
    domains::product::{
        domain::{repository::ProductRepository, service::ProductServiceTrait},
        dto::product_dto::ProductDto,
        infra::impl_repository::ProductRepo,
    },
};
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProductService {
    pub pool: PgPool,
    pub repo: Arc<dyn ProductRepository + Send + Sync>,
}

#[async_trait]
impl ProductServiceTrait for ProductService {
    /// constructor for the service.
    fn create_service(pool: PgPool) -> Arc<dyn ProductServiceTrait> {
        Arc::new(Self {
            pool,
            repo: Arc::new(ProductRepo {}),
        })
    }

    async fn get_product_by_id(&self, id: i32) -> Result<ProductDto, AppError> {
        match self.repo.find_by_id(self.pool.clone(), id).await {
            Ok(Some(product)) => Ok(ProductDto::from(product)),
            Ok(None) => Err(AppError::NotFound("Product not found".into())),
            Err(err) => {
                tracing::error!("Error retrieving product: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    async fn get_products(&self) -> Result<Vec<ProductDto>, AppError> {
        match self.repo.find_all(self.pool.clone()).await {
            Ok(products) => {
                let product_dtos: Vec<ProductDto> = products.into_iter().map(Into::into).collect();
                Ok(product_dtos)
            }
            Err(err) => {
                tracing::error!("Error fetching products: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    async fn get_products_by_category_id(
        &self,
        category_id: i32,
    ) -> Result<Vec<ProductDto>, AppError> {
        match self
            .repo
            .find_by_category_id(self.pool.clone(), category_id)
            .await
        {
            Ok(products) => {
                let product_dtos: Vec<ProductDto> = products.into_iter().map(Into::into).collect();
                Ok(product_dtos)
            }
            Err(err) => {
                tracing::error!("Error fetching products by category: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    async fn get_best_sellers(&self, limit: i64) -> Result<Vec<ProductDto>, AppError> {
        match self.repo.find_best_sellers(self.pool.clone(), limit).await {
            Ok(products) => {
                let product_dtos: Vec<ProductDto> = products.into_iter().map(Into::into).collect();
                Ok(product_dtos)
            }
            Err(err) => {
                tracing::error!("Error fetching best sellers: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    async fn get_deals_of_the_day(&self, limit: i64) -> Result<Vec<ProductDto>, AppError> {
        match self
            .repo
            .find_deals_of_the_day(self.pool.clone(), limit)
            .await
        {
            Ok(products) => {
                let product_dtos: Vec<ProductDto> = products.into_iter().map(Into::into).collect();
                Ok(product_dtos)
            }
            Err(err) => {
                tracing::error!("Error fetching deals of the day: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    async fn get_products_by_price_range(
        &self,
        min_price: BigDecimal,
        max_price: BigDecimal,
    ) -> Result<Vec<ProductDto>, AppError> {
        match self
            .repo
            .find_by_price_range(self.pool.clone(), min_price, max_price)
            .await
        {
            Ok(products) => {
                let product_dtos: Vec<ProductDto> = products.into_iter().map(Into::into).collect();
                Ok(product_dtos)
            }
            Err(err) => {
                tracing::error!("Error fetching products by price range: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }
}
