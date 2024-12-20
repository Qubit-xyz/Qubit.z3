pub mod agent;
pub mod cli_chatbot;
pub mod completion;
pub mod embeddings;
pub mod extractor;
pub(crate) mod json_utils;
pub mod loaders;
pub mod one_or_many;
pub mod pipeline;
pub mod providers;
pub mod tool;
pub mod vector_store;

// Re-export commonly used types and traits
pub use embeddings::Embed;
pub use one_or_many::{EmptyListError, OneOrMany};

#[cfg(feature = "derive")]
pub use rig_derive::Embed;
