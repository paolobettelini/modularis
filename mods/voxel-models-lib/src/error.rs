use std::path::PathBuf;

use crate::ResourceLocation;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid resource location `{0}`")]
    InvalidResourceLocation(String),

    #[error("resource path escapes its root: `{0}`")]
    UnsafeResourcePath(String),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("I/O error while reading `{path}`: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("model `{0}` was not found")]
    ModelNotFound(ResourceLocation),

    #[error("blockstate `{0}` was not found")]
    BlockStateNotFound(ResourceLocation),

    #[error("item definition `{0}` was not found")]
    ItemDefinitionNotFound(ResourceLocation),

    #[error("model parent cycle: {0}")]
    ParentCycle(String),

    #[error("model parent chain exceeded {0} entries")]
    ParentDepthExceeded(usize),

    #[error("texture reference cycle while resolving `{0}`")]
    TextureCycle(String),

    #[error("missing texture variable `{0}`")]
    MissingTextureVariable(String),

    #[error("invalid blockstate variant key `{0}`")]
    InvalidVariantKey(String),

    #[error("unsupported or malformed document: {0}")]
    UnsupportedDocument(String),
}
