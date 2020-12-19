//! Definitions for the request handlers.

use crate::{core::error, lsp::server::Server, provider, service::synchronizer};
use lspower::{jsonrpc::Result, lsp_types::*, LanguageServer};

#[lspower::async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // Receive and store the client capabilities.
        *self.session.client_capabilities.write().await = Some(params.capabilities);
        // Return the server capabilities.
        let capabilities = crate::lsp::server::capabilities();
        Ok(InitializeResult {
            capabilities,
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        let typ = MessageType::Info;
        let message = "WebAssembly language server initialized!";
        self.client.log_message(typ, message).await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let session = self.session.clone();
        synchronizer::document::open(session, params).await.unwrap()
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let session = self.session.clone();
        synchronizer::document::change(session, params).await.unwrap()
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let session = self.session.clone();
        synchronizer::document::close(session, params).await.unwrap()
    }

    async fn document_symbol(&self, params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        let session = self.session.clone();
        let result = provider::document_symbol(session, params).await;
        Ok(result.map_err(error::IntoJsonRpcError)?)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let session = self.session.clone();
        let result = provider::hover(session, params).await;
        Ok(result.map_err(error::IntoJsonRpcError)?)
    }

    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> Result<Option<SemanticTokensResult>> {
        let session = self.session.clone();
        let result = provider::semantic_tokens_full(session, params).await;
        Ok(result.map_err(error::IntoJsonRpcError)?)
    }

    async fn semantic_tokens_full_delta(
        &self,
        params: SemanticTokensDeltaParams,
    ) -> Result<Option<SemanticTokensFullDeltaResult>> {
        let _ = params;
        log::info!("Got a textDocument/semanticTokens/full/delta request, but it is not implemented");
        Ok(None)
    }

    async fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        let session = self.session.clone();
        let result = provider::semantic_tokens_range(session, params).await;
        Ok(result.map_err(error::IntoJsonRpcError)?)
    }

    async fn semantic_tokens_refresh(&self) -> Result<()> {
        log::info!("Got a textDocument/semanticTokens/refresh request, but it is not implemented");
        Ok(())
    }
}
