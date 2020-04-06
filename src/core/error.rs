//! Core functionality related to runtime errors.

use thiserror::Error;
use tree_sitter;

/// Convenience type for computations that may fail with an error.
pub(crate) type Fallible<T> = anyhow::Result<T>;

/// Runtime errors for the WASM language server.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub(crate) enum Error {
    /// Error that occurs when parsing an invalid language-id string.
    #[error("core::InvalidLanguageId: {0}")]
    CoreInvalidLanguageId(String),
    /// Error that occurs when attempting to set an invalid language for a tree-sitter parser.
    #[error("tree_sitter::LanguageError: {0}")]
    TreeSitterLanguageError(tree_sitter::LanguageError),
    /// Error that occurs when attempting to create a tree-sitter query from invalid source.
    #[error("tree_sitter::QueryError")]
    TreeSitterQueryError(tree_sitter::QueryError),
}

/// Convenience newtype wrapper for convertion to jsonrpc_core::Error.
pub(crate) struct IntoJsonRpcError(pub(crate) anyhow::Error);

impl From<IntoJsonRpcError> for jsonrpc_core::Error {
    fn from(error: IntoJsonRpcError) -> Self {
        let mut rpc_error = jsonrpc_core::error::Error::internal_error();
        rpc_error.message = format!("{}", error.0);
        rpc_error
    }
}
