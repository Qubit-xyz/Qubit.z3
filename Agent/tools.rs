use std::{collections::HashMap, pin::Pin};

use futures::Future;
use serde::{Deserialize, Serialize};

use crate::{
    completion::{self, ToolDefinition},
    embeddings::{embed::EmbedError, tool::ToolSchema},
};

#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("ToolCallError: {0}")]
    ToolCallError(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("JsonError: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub trait Tool: Sized + Send + Sync {

    const NAME: &'static str;

    type Error: std::error::Error + Send + Sync + 'static;
    type Args: for<'a> Deserialize<'a> + Send + Sync;
    type Output: Serialize;
    fn name(&self) -> String {
        Self::NAME.to_string()
    }

    fn definition(&self, _prompt: String) -> impl Future<Output = ToolDefinition> + Send + Sync;

    fn call(
        &self,
        args: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send + Sync;
}

pub trait ToolEmbedding: Tool {
    type InitError: std::error::Error + Send + Sync + 'static;

    type Context: for<'a> Deserialize<'a> + Serialize;

    type State: Send;

    fn embedding_docs(&self) -> Vec<String>;

    fn context(&self) -> Self::Context;

    fn init(state: Self::State, context: Self::Context) -> Result<Self, Self::InitError>;
}

pub trait ToolDyn: Send + Sync {
    fn name(&self) -> String;

    fn definition(
        &self,
        prompt: String,
    ) -> Pin<Box<dyn Future<Output = ToolDefinition> + Send + Sync + '_>>;

    fn call(
        &self,
        args: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, ToolError>> + Send + Sync + '_>>;
}

impl<T: Tool> ToolDyn for T {
    fn name(&self) -> String {
        self.name()
    }

    fn definition(
        &self,
        prompt: String,
    ) -> Pin<Box<dyn Future<Output = ToolDefinition> + Send + Sync + '_>> {
        Box::pin(<Self as Tool>::definition(self, prompt))
    }

    fn call(
        &self,
        args: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, ToolError>> + Send + Sync + '_>> {
        Box::pin(async move {
            match serde_json::from_str(&args) {
                Ok(args) => <Self as Tool>::call(self, args)
                    .await
                    .map_err(|e| ToolError::ToolCallError(Box::new(e)))
                    .and_then(|output| {
                        serde_json::to_string(&output).map_err(ToolError::JsonError)
                    }),
                Err(e) => Err(ToolError::JsonError(e)),
            }
        })
    }
}

pub trait ToolEmbeddingDyn: ToolDyn {
    fn context(&self) -> serde_json::Result<serde_json::Value>;

    fn embedding_docs(&self) -> Vec<String>;
}

impl<T: ToolEmbedding> ToolEmbeddingDyn for T {
    fn context(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self.context())
    }

    fn embedding_docs(&self) -> Vec<String> {
        self.embedding_docs()
    }
}

pub(crate) enum ToolType {
    Simple(Box<dyn ToolDyn>),
    Embedding(Box<dyn ToolEmbeddingDyn>),
}

impl ToolType {
    pub fn name(&self) -> String {
        match self {
            ToolType::Simple(tool) => tool.name(),
            ToolType::Embedding(tool) => tool.name(),
        }
    }

    pub async fn definition(&self, prompt: String) -> ToolDefinition {
        match self {
            ToolType::Simple(tool) => tool.definition(prompt).await,
            ToolType::Embedding(tool) => tool.definition(prompt).await,
        }
    }

    pub async fn call(&self, args: String) -> Result<String, ToolError> {
        match self {
            ToolType::Simple(tool) => tool.call(args).await,
            ToolType::Embedding(tool) => tool.call(args).await,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ToolSetError {

    #[error("ToolCallError: {0}")]
    ToolCallError(#[from] ToolError),

    #[error("ToolNotFoundError: {0}")]
    ToolNotFoundError(String),


    #[error("JsonError: {0}")]
    JsonError(#[from] serde_json::Error),
}


#[derive(Default)]
pub struct ToolSet {
    pub(crate) tools: HashMap<String, ToolType>,
}

impl ToolSet {

    pub fn from_tools(tools: Vec<impl ToolDyn + 'static>) -> Self {
        let mut toolset = Self::default();
        tools.into_iter().for_each(|tool| {
            toolset.add_tool(tool);
        });
        toolset
    }


    pub fn builder() -> ToolSetBuilder {
        ToolSetBuilder::default()
    }


    pub fn contains(&self, toolname: &str) -> bool {
        self.tools.contains_key(toolname)
    }


    pub fn add_tool(&mut self, tool: impl ToolDyn + 'static) {
        self.tools
            .insert(tool.name(), ToolType::Simple(Box::new(tool)));
    }


    pub fn add_tools(&mut self, toolset: ToolSet) {
        self.tools.extend(toolset.tools);
    }

    pub(crate) fn get(&self, toolname: &str) -> Option<&ToolType> {
        self.tools.get(toolname)
    }


    pub async fn call(&self, toolname: &str, args: String) -> Result<String, ToolSetError> {
        if let Some(tool) = self.tools.get(toolname) {
            tracing::info!(target: "rig",
                "Calling tool {toolname} with args:\n{}",
                serde_json::to_string_pretty(&args).unwrap_or_else(|_| args.clone())
            );
            Ok(tool.call(args).await?)
        } else {
            Err(ToolSetError::ToolNotFoundError(toolname.to_string()))
        }
    }


    pub async fn documents(&self) -> Result<Vec<completion::Document>, ToolSetError> {
        let mut docs = Vec::new();
        for tool in self.tools.values() {
            match tool {
                ToolType::Simple(tool) => {
                    docs.push(completion::Document {
                        id: tool.name(),
                        text: format!(
                            "\
                            Tool: {}\n\
                            Definition: \n\
                            {}\
                        ",
                            tool.name(),
                            serde_json::to_string_pretty(&tool.definition("".to_string()).await)?
                        ),
                        additional_props: HashMap::new(),
                    });
                }
                ToolType::Embedding(tool) => {
                    docs.push(completion::Document {
                        id: tool.name(),
                        text: format!(
                            "\
                            Tool: {}\n\
                            Definition: \n\
                            {}\
                        ",
                            tool.name(),
                            serde_json::to_string_pretty(&tool.definition("".to_string()).await)?
                        ),
                        additional_props: HashMap::new(),
                    });
                }
            }
        }
        Ok(docs)
    }

    pub fn schemas(&self) -> Result<Vec<ToolSchema>, EmbedError> {
        self.tools
            .values()
            .filter_map(|tool_type| {
                if let ToolType::Embedding(tool) = tool_type {
                    Some(ToolSchema::try_from(&**tool))
                } else {
                    None
                }
            })
            .collect::<Result<Vec<_>, _>>()
    }
}

#[derive(Default)]
pub struct ToolSetBuilder {
    tools: Vec<ToolType>,
}

impl ToolSetBuilder {
    pub fn static_tool(mut self, tool: impl ToolDyn + 'static) -> Self {
        self.tools.push(ToolType::Simple(Box::new(tool)));
        self
    }

    pub fn dynamic_tool(mut self, tool: impl ToolEmbeddingDyn + 'static) -> Self {
        self.tools.push(ToolType::Embedding(Box::new(tool)));
        self
    }

    pub fn build(self) -> ToolSet {
        ToolSet {
            tools: self
                .tools
                .into_iter()
                .map(|tool| (tool.name(), tool))
                .collect(),
        }
    }
}
