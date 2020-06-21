use async_trait::async_trait;

pub trait TQuery<TResponse> {}

#[async_trait]
pub trait TQueryHandler<TQuery, TResponse, TError> {
    async fn get(&self) -> Result<TResponse, TError>;
}
