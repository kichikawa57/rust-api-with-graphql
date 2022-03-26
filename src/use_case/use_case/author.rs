use async_trait::async_trait;
use mockall::automock;

use crate::use_case::{
    dto::author::{Author, CreateAuthorData},
    error::UseCaseError,
};

#[automock]
#[async_trait]
pub trait CreateAuthorUseCase: Send + Sync + 'static {
    async fn create(
        &self,
        user_id: &str,
        author_data: CreateAuthorData,
    ) -> Result<Author, UseCaseError>;
}
