use async_trait::async_trait;

pub trait TCommand<TResult> {}

#[async_trait]
pub trait TCommandHandler<TCommand, TResult> {
    async fn execute(&mut self) -> TResult;
}
