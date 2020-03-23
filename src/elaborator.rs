use crate::{database::Database, message::Message, synchronizer::Synchronizer};
use failure::Fallible;
use lsp_types::*;
use std::sync::Arc;
use tokio::sync::watch::Receiver;
use tower_lsp::Client;

/// Elaborates parse trees into structured data to be cached in the database.
pub struct Elaborator {
    database: Arc<Database>,
    receiver: Receiver<Message>,
    synchronizer: Arc<Synchronizer>,
}

impl Elaborator {
    pub fn new(
        database: Arc<Database>,
        receiver: Receiver<Message>,
        synchronizer: Arc<Synchronizer>,
    ) -> Fallible<Self> {
        Ok(Elaborator {
            database,
            receiver,
            synchronizer,
        })
    }

    pub async fn init(&self) -> Fallible<()> {
        let mut receiver = self.receiver.clone();
        while let Some(message) = receiver.recv().await {
            match message {
                Message::TreeDidChange { client, uri, .. } => self.tree_did_change(client, uri).await?,
                Message::TreeDidClose { client, uri, .. } => self.tree_did_close(client, uri).await?,
                Message::TreeDidOpen { client, uri, .. } => self.tree_did_open(client, uri).await?,
                _ => {},
            }
        }
        Ok(())
    }

    async fn tree_did_change(&self, _: Client, uri: Url) -> Fallible<()> {
        if let Some(tree) = self.synchronizer.trees.get(&uri) {
            let tree = tree.lock().await.clone();
            let node = tree.root_node();
            if !node.has_error() {
                log::info!("syntax well-formed");
            }
            // NOTE: else let auditor handle
            // TODO: allow partial elaboration in presence of syntax errors
        }
        Ok(())
    }

    async fn tree_did_close(&self, _: Client, _: Url) -> Fallible<()> {
        Ok(())
    }

    async fn tree_did_open(&self, _: Client, uri: Url) -> Fallible<()> {
        if let Some(tree) = self.synchronizer.trees.get(&uri) {
            let tree = tree.lock().await.clone();
            let node = tree.root_node();
            if !node.has_error() {
                log::info!("syntax well-formed");
            }
            // NOTE: else let auditor handle
            // TODO: allow partial elaboration in presence of syntax errors
        }
        Ok(())
    }
}
