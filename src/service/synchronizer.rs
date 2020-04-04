//! Synchronizes document changes between editor and server

/// Functions related to processing events for a document.
pub(crate) mod document {
    use crate::core::session::Session;
    use failure::Fallible;
    use lsp_types::*;
    use std::sync::Arc;
    use tower_lsp::Client;

    /// Handle a document "change" event.
    pub(crate) async fn change(
        session: Arc<Session>,
        client: &'static Client,
        params: DidChangeTextDocumentParams,
    ) -> Fallible<()> {
        let DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier { uri, .. },
            content_changes,
        } = params;
        let TextDocumentContentChangeEvent { text, .. } = content_changes[0].clone();

        let tree_was_generated = super::tree::change(session.clone(), uri.clone(), text).await;

        // on successful generation of a parse tree (which may contain syntax errors)
        if tree_was_generated {
            // run the auditor tasks
            crate::service::auditor::tree::change(session.clone(), client, uri.clone()).await?;

            // run the elaborator tasks
            crate::service::elaborator::tree::change(session.clone(), client, uri.clone()).await?;
        } else {
            // TODO: report
            log::warn!("tree_was_generated == false");
        }
        Ok(())
    }

    /// Handle a document "close" event.
    pub(crate) async fn close(
        session: Arc<Session>,
        client: &'static Client,
        params: DidCloseTextDocumentParams,
    ) -> Fallible<()> {
        let DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri },
        } = &params;
        session.documents.remove(uri);
        crate::service::auditor::tree::close(session.clone(), client, uri.clone()).await?;
        crate::service::elaborator::tree::close(session.clone(), client, uri.clone()).await?;
        Ok(())
    }

    /// Handle a document "open" event.
    pub(crate) async fn open(
        session: Arc<Session>,
        client: &'static Client,
        params: DidOpenTextDocumentParams,
    ) -> Fallible<()> {
        let DidOpenTextDocumentParams {
            text_document: TextDocumentItem { uri, .. },
        } = &params;

        // spawn a parser and try to generate a syntax tree
        let tree_was_generated = super::tree::open(session.clone(), params.clone()).await?;

        // on successful generation of a parse tree (which may contain syntax errors)
        if tree_was_generated {
            // run the auditor tasks
            crate::service::auditor::tree::open(session.clone(), client, uri.clone()).await?;
            // run the elaborator tasks
            crate::service::elaborator::tree::open(session.clone(), client, uri.clone()).await?;
        } else {
            // TODO: report
            log::warn!("tree_was_generated == false");
        }
        Ok(())
    }
}

/// Functions related to processing parse tree events for a document.
mod tree {
    use crate::core::{document::Document, session::Session};
    use failure::Fallible;
    use lsp_types::*;
    use std::{convert::TryFrom, sync::Arc};
    use tokio::sync::Mutex;

    // TODO: implement parser cancellation
    /// Handle a parse tree "change" event.
    pub(super) async fn change(session: Arc<Session>, uri: Url, text: String) -> bool {
        let mut success = false;
        if let Some(mut document) = session.documents.get_mut(&uri) {
            let tree;
            {
                let mut parser = document.parser.lock().await;
                // FIXME: we reset the parser since we don't handle incremental changes yet
                parser.reset();
                // TODO: Fetch old_tree from cache and apply edits to prepare for incremental re-parsing.
                let old_tree = None;
                tree = parser.parse(&text[..], old_tree);
            }
            if let Some(tree) = tree {
                document.text = text;
                document.tree = Mutex::new(tree);
                success = true;
            }
        }
        success
    }

    // TODO: implement parser cancellation
    /// Handle a parse tree "open" event.
    pub(super) async fn open(session: Arc<Session>, params: DidOpenTextDocumentParams) -> Fallible<bool> {
        let DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                language_id, text, uri, ..
            },
        } = params;

        let language = crate::core::language::Language::try_from(language_id)?;
        let mut parser = crate::core::parser::try_from(language)?;

        // TODO: Fetch old_tree from cache and apply edits to prepare for incremental re-parsing.
        let old_tree = None;

        let mut success = false;
        if let Some(tree) = parser.parse(&text[..], old_tree) {
            session.documents.insert(uri, Document {
                language,
                parser: Mutex::new(parser),
                text: text.clone(),
                tree: Mutex::new(tree),
            });
            success = true;
        }
        Ok(success)
    }
}
