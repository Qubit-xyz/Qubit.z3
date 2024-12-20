pub mod agent_ops;
pub mod op;
pub mod try_op;
#[macro_use]
pub mod parallel;

use std::future::Future;

pub use op::{map, passthrough, then, Op};
pub use try_op::TryOp;

use crate::{completion, extractor::Extractor, vector_store};

pub struct PipelineBuilder<E> {
    _error: std::marker::PhantomData<E>,
}

impl<E> PipelineBuilder<E> {
    pub fn map<F, Input, Output>(self, f: F) -> op::Map<F, Input>
    where
        F: Fn(Input) -> Output + Send + Sync,
        Input: Send + Sync,
        Output: Send + Sync,
        Self: Sized,
    {
        op::Map::new(f)
    }

    pub fn then<F, Input, Fut>(self, f: F) -> op::Then<F, Input>
    where
        F: Fn(Input) -> Fut + Send + Sync,
        Input: Send + Sync,
        Fut: Future + Send + Sync,
        Fut::Output: Send + Sync,
        Self: Sized,
    {
        op::Then::new(f)
    }

   
    pub fn chain<T>(self, op: T) -> T
    where
        T: Op,
        Self: Sized,
    {
        op
    }

    pub fn lookup<I, Input, Output>(self, index: I, n: usize) -> agent_ops::Lookup<I, Input, Output>
    where
        I: vector_store::VectorStoreIndex,
        Output: Send + Sync + for<'a> serde::Deserialize<'a>,
        Input: Into<String> + Send + Sync,
        // E: From<vector_store::VectorStoreError> + Send + Sync,
        Self: Sized,
    {
        agent_ops::Lookup::new(index, n)
    }

    pub fn prompt<P, Input>(self, agent: P) -> agent_ops::Prompt<P, Input>
    where
        P: completion::Prompt,
        Input: Into<String> + Send + Sync,
        // E: From<completion::PromptError> + Send + Sync,
        Self: Sized,
    {
        agent_ops::Prompt::new(agent)
    }

    pub fn extract<M, Input, Output>(
        self,
        extractor: Extractor<M, Output>,
    ) -> agent_ops::Extract<M, Input, Output>
    where
        M: completion::CompletionModel,
        Output: schemars::JsonSchema + for<'a> serde::Deserialize<'a> + Send + Sync,
        Input: Into<String> + Send + Sync,
    {
        agent_ops::Extract::new(extractor)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ChainError {
    #[error("Failed to prompt agent: {0}")]
    PromptError(#[from] completion::PromptError),

    #[error("Failed to lookup documents: {0}")]
    LookupError(#[from] vector_store::VectorStoreError),
}

pub fn new() -> PipelineBuilder<ChainError> {
    PipelineBuilder {
        _error: std::marker::PhantomData,
    }
}

pub fn with_error<E>() -> PipelineBuilder<E> {
    PipelineBuilder {
        _error: std::marker::PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent_ops::tests::{Foo, MockIndex, MockModel};
    use parallel::parallel;

    #[tokio::test]
    async fn test_prompt_pipeline() {
        let model = MockModel;

        let chain = super::new()
            .map(|input| format!("User query: {}", input))
            .prompt(model);

        let result = chain
            .call("What is a flurbo?")
            .await
            .expect("Failed to run chain");

        assert_eq!(result, "Mock response: User query: What is a flurbo?");
    }

    #[tokio::test]
    async fn test_prompt_pipeline_error() {
        let model = MockModel;

        let chain = super::with_error::<()>()
            .map(|input| format!("User query: {}", input))
            .prompt(model);

        let result = chain
            .try_call("What is a flurbo?")
            .await
            .expect("Failed to run chain");

        assert_eq!(result, "Mock response: User query: What is a flurbo?");
    }

    #[tokio::test]
    async fn test_lookup_pipeline() {
        let index = MockIndex;

        let chain = super::new()
            .lookup::<_, _, Foo>(index, 1)
            .map_ok(|docs| format!("Top documents:\n{}", docs[0].2.foo));

        let result = chain
            .try_call("What is a flurbo?")
            .await
            .expect("Failed to run chain");

        assert_eq!(result, "Top documents:\nbar");
    }

    #[tokio::test]
    async fn test_rag_pipeline() {
        let index = MockIndex;

        let chain = super::new()
            .chain(parallel!(
                passthrough(),
                agent_ops::lookup::<_, _, Foo>(index, 1),
            ))
            .map(|(query, maybe_docs)| match maybe_docs {
                Ok(docs) => format!("User query: {}\n\nTop documents:\n{}", query, docs[0].2.foo),
                Err(err) => format!("Error: {}", err),
            })
            .prompt(MockModel);

        let result = chain
            .call("What is a flurbo?")
            .await
            .expect("Failed to run chain");

        assert_eq!(
            result,
            "Mock response: User query: What is a flurbo?\n\nTop documents:\nbar"
        );
    }
}
