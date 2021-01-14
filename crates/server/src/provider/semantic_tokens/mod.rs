//! Provides `textDocument/semanticTokens/*` functionality.

mod builder;
mod node_walker;

/// Semantic tokens provider definitions for ".wast" files.
pub mod wast {
    use super::{builder::SemanticTokensBuilder, node_walker::NodeWalker};
    use crate::core::{
        self,
        language::{wast, Language},
    };
    use anyhow::anyhow;
    use std::sync::Arc;

    struct Handler<'a> {
        builder: SemanticTokensBuilder<'a>,
        walker: NodeWalker<'a>,
    }

    impl<'a> Handler<'a> {
        fn new(language: Language, legend: Option<&'a lsp::SemanticTokensLegend>, node: tree_sitter::Node<'a>) -> Self {
            let builder = SemanticTokensBuilder::new(legend);
            let walker = NodeWalker::new(language, node);
            Self { builder, walker }
        }
    }

    pub(crate) async fn full(
        session: Arc<core::Session>,
        params: lsp::SemanticTokensParams,
        document: &core::Document,
    ) -> anyhow::Result<Option<lsp::SemanticTokensResult>> {
        let params = {
            let tree = session.get_tree(&params.text_document.uri).await?;
            lsp::SemanticTokensRangeParams {
                work_done_progress_params: params.work_done_progress_params,
                partial_result_params: params.partial_result_params,
                text_document: params.text_document,
                range: {
                    let tree = tree.lock().await;
                    let node = tree.root_node();
                    crate::util::node::range(&node)
                },
            }
        };

        let result = range(session, params, document).await?.map(|result| match result {
            lsp::SemanticTokensRangeResult::Tokens(tokens) => lsp::SemanticTokensResult::Tokens(tokens),
            lsp::SemanticTokensRangeResult::Partial(partial) => lsp::SemanticTokensResult::Partial(partial),
        });

        Ok(result)
    }

    pub(crate) async fn range(
        session: Arc<core::Session>,
        params: lsp::SemanticTokensRangeParams,
        document: &core::Document,
    ) -> anyhow::Result<Option<lsp::SemanticTokensRangeResult>> {
        let language = document.language;
        let legend = session.semantic_tokens_legend().await;
        let legend = legend.as_ref();

        let tree = session.get_tree(&params.text_document.uri).await?;
        let tree = tree.lock().await;

        if let Some(node) = {
            let start = tree_sitter::Point {
                row: params.range.start.line as usize,
                column: params.range.start.character as usize,
            };
            let end = tree_sitter::Point {
                row: params.range.end.line as usize,
                column: params.range.end.character as usize,
            };
            tree.root_node().descendant_for_point_range(start, end)
        } {
            let mut handler = Handler::new(language, legend, node);

            loop {
                if handler.walker.done {
                    break;
                }

                // handle "root"
                if wast::kind::equals::ROOT(handler.walker.kind()) {
                    handler.root();
                    continue;
                }

                // handle "_action"
                if wast::kind::equals::ACTION_GET(handler.walker.kind()) {
                    handler.action_get()?;
                    continue;
                } else if wast::kind::token::equals::GET(handler.walker.kind()) {
                    handler.token_get()?;
                    continue;
                } else if wast::kind::equals::ACTION_INVOKE(handler.walker.kind()) {
                    handler.action_invoke()?;
                    continue;
                } else if wast::kind::token::equals::INVOKE(handler.walker.kind()) {
                    handler.token_invoke()?;
                    continue;
                }

                // handle "_assertion"
                if wast::kind::equals::ASSERT_EXHAUSTION(handler.walker.kind()) {
                    handler.assert_exhaustion()?;
                    continue;
                } else if wast::kind::equals::ASSERT_INVALID(handler.walker.kind()) {
                    handler.assert_invalid()?;
                    continue;
                } else if wast::kind::equals::ASSERT_EXHAUSTION(handler.walker.kind()) {
                    handler.assert_exhaustion()?;
                    continue;
                } else if wast::kind::equals::ASSERT_INVALID(handler.walker.kind()) {
                    handler.assert_invalid()?;
                    continue;
                } else if wast::kind::equals::ASSERT_MALFORMED(handler.walker.kind()) {
                    handler.assert_malformed()?;
                    continue;
                } else if wast::kind::equals::ASSERT_RETURN(handler.walker.kind()) {
                    handler.assert_return()?;
                    continue;
                } else if wast::kind::equals::ASSERT_RETURN_ARITHMETIC_NAN(handler.walker.kind()) {
                    handler.assert_return_arithmetic_nan()?;
                    continue;
                } else if wast::kind::equals::ASSERT_RETURN_CANONICAL_NAN(handler.walker.kind()) {
                    handler.assert_return_canonical_nan()?;
                    continue;
                } else if wast::kind::equals::ASSERT_TRAP_ACTION(handler.walker.kind()) {
                    handler.assert_trap_action()?;
                    continue;
                } else if wast::kind::equals::ASSERT_TRAP_MODULE(handler.walker.kind()) {
                    handler.assert_trap_module()?;
                    continue;
                } else if wast::kind::equals::ASSERT_UNLINKABLE(handler.walker.kind()) {
                    handler.assert_unlinkable()?;
                    continue;
                }

                // handle "command"
                if wast::kind::equals::COMMAND(handler.walker.kind()) {
                    handler.command();
                    continue;
                }

                // handle {"comment_block", "comment_block_annot", "comment_line", "comment_line_annot"}
                if wast::kind::equals::COMMENT_BLOCK(handler.walker.kind()) {
                    handler.comment_block()?;
                    continue;
                } else if wast::kind::equals::COMMENT_BLOCK_ANNOT(handler.walker.kind()) {
                    handler.comment_block_annot()?;
                    continue;
                } else if wast::kind::equals::COMMENT_LINE(handler.walker.kind()) {
                    handler.comment_line()?;
                    continue;
                } else if wast::kind::equals::COMMENT_LINE_ANNOT(handler.walker.kind()) {
                    handler.comment_line_annot()?;
                    continue;
                }

                // handle "_meta"
                if wast::kind::equals::META_INPUT(handler.walker.kind()) {
                    handler.meta_input()?;
                    continue;
                } else if wast::kind::equals::META_OUTPUT(handler.walker.kind()) {
                    handler.meta_output()?;
                    continue;
                } else if wast::kind::equals::META_SCRIPT(handler.walker.kind()) {
                    handler.meta_script()?;
                    continue;
                }

                // handle "module"
                if wast::kind::equals::MODULE(handler.walker.kind()) {
                    handler.module()?;
                    continue;
                }

                // handle "_module_field"
                if wast::kind::equals::MODULE_FIELD_DATA(handler.walker.kind()) {
                    handler.module_field_data()?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_ELEM(handler.walker.kind()) {
                    handler.module_field_elem()?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_EXPORT(handler.walker.kind()) {
                    handler.module_field_export()?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_FUNC(handler.walker.kind()) {
                    handler.module_field_func()?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_GLOBAL(handler.walker.kind()) {
                    handler.module_field_global()?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_IMPORT(handler.walker.kind()) {
                    handler.module_field_import()?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_MEMORY(handler.walker.kind()) {
                    handler.module_field_memory()?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_START(handler.walker.kind()) {
                    handler.module_field_start()?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_TABLE(handler.walker.kind()) {
                    handler.module_field_table()?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_TYPE(handler.walker.kind()) {
                    handler.module_field_type()?;
                    continue;
                }

                // handle "register"
                if wast::kind::equals::REGISTER(handler.walker.kind()) {
                    handler.register()?;
                    continue;
                }

                // handle "_script_module"
                if wast::kind::equals::MODULE(handler.walker.kind()) {
                    // handled earlier
                    unreachable!();
                } else if wast::kind::equals::SCRIPT_MODULE_BINARY(handler.walker.kind()) {
                    handler.script_module_binary()?;
                    continue;
                } else if wast::kind::equals::SCRIPT_MODULE_QUOTE(handler.walker.kind()) {
                    handler.script_module_quote()?;
                    continue;
                }

                // catch all case
                handler.walker.goto_next();
            }

            let tokens = handler.builder.build();
            let result = lsp::SemanticTokensRangeResult::Tokens(tokens);

            Ok(Some(result))
        } else {
            Err(anyhow!("Could not obtain tree node for given range"))
        }
    }

    impl<'a> Handler<'a> {
        fn action_get(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn action_invoke(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn assert_exhaustion(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn assert_invalid(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn assert_malformed(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn assert_return(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn assert_return_arithmetic_nan(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn assert_return_canonical_nan(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn assert_trap_action(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn assert_trap_module(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn assert_unlinkable(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn command(&mut self) {
            self.walker.goto_first_child();
        }

        fn comment_block(&mut self) -> anyhow::Result<()> {
            let range = crate::util::node::range(&self.walker.node());
            self.builder.push(range, &lsp::SemanticTokenType::COMMENT, None)?;

            self.walker.goto_next();

            Ok(())
        }

        fn comment_block_annot(&mut self) -> anyhow::Result<()> {
            let range = crate::util::node::range(&self.walker.node());
            self.builder.push(range, &lsp::SemanticTokenType::COMMENT, None)?;

            self.walker.goto_next();

            Ok(())
        }

        fn comment_line(&mut self) -> anyhow::Result<()> {
            let range = crate::util::node::range(&self.walker.node());
            self.builder.push(range, &lsp::SemanticTokenType::COMMENT, None)?;

            self.walker.goto_next();

            Ok(())
        }

        fn comment_line_annot(&mut self) -> anyhow::Result<()> {
            let range = crate::util::node::range(&self.walker.node());
            self.builder.push(range, &lsp::SemanticTokenType::COMMENT, None)?;

            self.walker.goto_next();

            Ok(())
        }

        fn export(&mut self) -> anyhow::Result<()> {
            // "("
            self.walker.goto_first_child();

            // "export"
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            // $.name
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::STRING, None)?;
            }

            // skip ")"
            self.walker.goto_next_sibling();

            self.walker.goto_next();

            Ok(())
        }

        fn import(&mut self) -> anyhow::Result<()> {
            // "("
            self.walker.goto_first_child();

            // "import"
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            // $.name
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::STRING, None)?;
            }

            // $.name
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::STRING, None)?;
            }

            // skip ")"
            self.walker.goto_next_sibling();

            self.walker.goto_next();

            Ok(())
        }

        fn meta_input(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn meta_output(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn meta_script(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_data(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_elem(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_export(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_func(&mut self) -> anyhow::Result<()> {
            // "("
            self.walker.goto_first_child();

            // "func"
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
                self.walker.goto_next_sibling();
            }

            // optional($.identifier)
            if wast::kind::equals::IDENTIFIER(self.walker.kind()) {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::FUNCTION, None)?;
                self.walker.goto_next_sibling();
            }

            // repeat($.export)
            while wast::kind::equals::EXPORT(self.walker.kind()) {
                self.export()?;
            }

            // optional($.import)
            if wast::kind::equals::IMPORT(self.walker.kind()) {
                self.import()?;
            }

            // optional($.type_use)
            if wast::kind::equals::TYPE_USE(self.walker.kind()) {
                self.type_use()?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_global(&mut self) -> anyhow::Result<()> {
            // "("
            self.walker.goto_first_child();

            // "global"
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
                self.walker.goto_next_sibling();
            }

            // optional($.identifier)
            if wast::kind::equals::IDENTIFIER(self.walker.kind()) {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::FUNCTION, None)?;
                self.walker.goto_next_sibling();
            }

            // repeat($.export)
            while wast::kind::equals::EXPORT(self.walker.kind()) {
                self.export()?;
            }

            // optional($.import)
            if wast::kind::equals::IMPORT(self.walker.kind()) {
                self.import()?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_import(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_memory(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_start(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_table(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_type(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn register(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn root(&mut self) {
            self.walker.goto_next();
        }

        fn script_module_binary(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn script_module_quote(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn type_use(&mut self) -> anyhow::Result<()> {
            // "("
            self.walker.goto_first_child();

            // "type"
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            // $.index
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::VARIABLE, None)?;
            }

            // skip ")"
            self.walker.goto_parent();

            self.walker.goto_next();

            Ok(())
        }

        fn token_get(&mut self) -> anyhow::Result<()> {
            let node = self.walker.node();
            let range = crate::util::node::range(&node);
            self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;

            self.walker.goto_next();

            Ok(())
        }

        fn token_invoke(&mut self) -> anyhow::Result<()> {
            let node = self.walker.node();
            let range = crate::util::node::range(&node);
            self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;

            self.walker.goto_next();

            Ok(())
        }
    }
}

/// Semantic tokens provider definitions for ".wat" files.
pub mod wat {
    use super::{builder::SemanticTokensBuilder, node_walker::NodeWalker};
    use crate::core::{
        self,
        language::{wat, Language},
    };
    use anyhow::anyhow;
    use std::sync::Arc;

    // Move to the next appropriate node in the syntax tree.
    struct Handler<'a> {
        builder: SemanticTokensBuilder<'a>,
        walker: NodeWalker<'a>,
    }

    impl<'a> Handler<'a> {
        fn new(language: Language, legend: Option<&'a lsp::SemanticTokensLegend>, node: tree_sitter::Node<'a>) -> Self {
            let builder = SemanticTokensBuilder::new(legend);
            let walker = NodeWalker::new(language, node);
            Self { builder, walker }
        }
    }

    pub(crate) async fn full(
        session: Arc<core::Session>,
        params: lsp::SemanticTokensParams,
        document: &core::Document,
    ) -> anyhow::Result<Option<lsp::SemanticTokensResult>> {
        let params = {
            let tree = session.get_tree(&params.text_document.uri).await?;
            lsp::SemanticTokensRangeParams {
                work_done_progress_params: params.work_done_progress_params,
                partial_result_params: params.partial_result_params,
                text_document: params.text_document,
                range: {
                    let tree = tree.lock().await;
                    let node = tree.root_node();
                    crate::util::node::range(&node)
                },
            }
        };

        let result = range(session, params, document).await?.map(|result| match result {
            lsp::SemanticTokensRangeResult::Tokens(tokens) => lsp::SemanticTokensResult::Tokens(tokens),
            lsp::SemanticTokensRangeResult::Partial(partial) => lsp::SemanticTokensResult::Partial(partial),
        });

        Ok(result)
    }

    pub(crate) async fn range(
        session: Arc<core::Session>,
        params: lsp::SemanticTokensRangeParams,
        document: &core::Document,
    ) -> anyhow::Result<Option<lsp::SemanticTokensRangeResult>> {
        let language = document.language;
        let legend = session.semantic_tokens_legend().await;
        let legend = legend.as_ref();

        let tree = session.get_tree(&params.text_document.uri).await?;
        let tree = tree.lock().await;

        if let Some(node) = {
            let start = tree_sitter::Point {
                row: params.range.start.line as usize,
                column: params.range.start.character as usize,
            };
            let end = tree_sitter::Point {
                row: params.range.end.line as usize,
                column: params.range.end.character as usize,
            };
            tree.root_node().descendant_for_point_range(start, end)
        } {
            let mut handler = Handler::new(language, legend, node);

            loop {
                if handler.walker.done {
                    break;
                }

                // handle "root"
                if wat::kind::equals::ROOT(handler.walker.kind()) {
                    handler.root();
                    continue;
                }

                // handle {"comment_block", "comment_block_annot", "comment_line", "comment_line_annot"}
                if wat::kind::equals::COMMENT_BLOCK(handler.walker.kind()) {
                    handler.comment_block()?;
                    continue;
                } else if wat::kind::equals::COMMENT_BLOCK_ANNOT(handler.walker.kind()) {
                    handler.comment_block_annot()?;
                    continue;
                } else if wat::kind::equals::COMMENT_LINE(handler.walker.kind()) {
                    handler.comment_line()?;
                    continue;
                } else if wat::kind::equals::COMMENT_LINE_ANNOT(handler.walker.kind()) {
                    handler.comment_line_annot()?;
                    continue;
                }

                // handle "module"
                if wat::kind::equals::MODULE(handler.walker.kind()) {
                    handler.module()?;
                    continue;
                }

                // handle "_module_field"
                if wat::kind::equals::MODULE_FIELD_DATA(handler.walker.kind()) {
                    handler.module_field_data()?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_ELEM(handler.walker.kind()) {
                    handler.module_field_elem()?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_EXPORT(handler.walker.kind()) {
                    handler.module_field_export()?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_FUNC(handler.walker.kind()) {
                    handler.module_field_func()?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_GLOBAL(handler.walker.kind()) {
                    handler.module_field_global()?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_IMPORT(handler.walker.kind()) {
                    handler.module_field_import()?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_MEMORY(handler.walker.kind()) {
                    handler.module_field_memory()?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_START(handler.walker.kind()) {
                    handler.module_field_start()?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_TABLE(handler.walker.kind()) {
                    handler.module_field_table()?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_TYPE(handler.walker.kind()) {
                    handler.module_field_type()?;
                    continue;
                }

                // FIXME: catch all case
                handler.walker.goto_next();
            }

            let tokens = handler.builder.build();
            let result = lsp::SemanticTokensRangeResult::Tokens(tokens);

            Ok(Some(result))
        } else {
            Err(anyhow!("Could not obtain tree node for given range"))
        }
    }

    impl<'a> Handler<'a> {
        fn comment_block(&mut self) -> anyhow::Result<()> {
            let range = crate::util::node::range(&self.walker.node());
            self.builder.push(range, &lsp::SemanticTokenType::COMMENT, None)?;

            self.walker.goto_next();

            Ok(())
        }

        fn comment_block_annot(&mut self) -> anyhow::Result<()> {
            let range = crate::util::node::range(&self.walker.node());
            self.builder.push(range, &lsp::SemanticTokenType::COMMENT, None)?;

            self.walker.goto_next();

            Ok(())
        }

        fn comment_line(&mut self) -> anyhow::Result<()> {
            let range = crate::util::node::range(&self.walker.node());
            self.builder.push(range, &lsp::SemanticTokenType::COMMENT, None)?;

            self.walker.goto_next();

            Ok(())
        }

        fn comment_line_annot(&mut self) -> anyhow::Result<()> {
            let range = crate::util::node::range(&self.walker.node());
            self.builder.push(range, &lsp::SemanticTokenType::COMMENT, None)?;

            self.walker.goto_next();

            Ok(())
        }

        fn export(&mut self) -> anyhow::Result<()> {
            // "("
            self.walker.goto_first_child();

            // "export"
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            // $.name
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::STRING, None)?;
            }

            // skip ")"
            self.walker.goto_next_sibling();

            self.walker.goto_next();

            Ok(())
        }

        fn import(&mut self) -> anyhow::Result<()> {
            // "("
            self.walker.goto_first_child();

            // "import"
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            // $.name
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::STRING, None)?;
            }

            // $.name
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::STRING, None)?;
            }

            // skip ")"
            self.walker.goto_next_sibling();

            self.walker.goto_next();

            Ok(())
        }

        fn module(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_data(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_elem(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_export(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_func(&mut self) -> anyhow::Result<()> {
            // "("
            self.walker.goto_first_child();

            // "func"
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
                self.walker.goto_next_sibling();
            }

            // optional($.identifier)
            if wat::kind::equals::IDENTIFIER(self.walker.kind()) {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::FUNCTION, None)?;
                self.walker.goto_next_sibling();
            }

            // repeat($.export)
            while wat::kind::equals::EXPORT(self.walker.kind()) {
                self.export()?;
            }

            // optional($.import)
            if wat::kind::equals::IMPORT(self.walker.kind()) {
                self.import()?;
            }

            // optional($.type_use)
            if wat::kind::equals::TYPE_USE(self.walker.kind()) {
                self.type_use()?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_global(&mut self) -> anyhow::Result<()> {
            // "("
            self.walker.goto_first_child();

            // "global"
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
                self.walker.goto_next_sibling();
            }

            // optional($.identifier)
            if wat::kind::equals::IDENTIFIER(self.walker.kind()) {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::FUNCTION, None)?;
                self.walker.goto_next_sibling();
            }

            // repeat($.export)
            while wat::kind::equals::EXPORT(self.walker.kind()) {
                self.export()?;
            }

            // optional($.import)
            if wat::kind::equals::IMPORT(self.walker.kind()) {
                self.import()?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_import(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_memory(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_start(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_table(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn module_field_type(&mut self) -> anyhow::Result<()> {
            if let Some(node) = self.walker.node().child(1) {
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            self.walker.goto_next();

            Ok(())
        }

        fn root(&mut self) {
            self.walker.goto_next();
        }

        fn type_use(&mut self) -> anyhow::Result<()> {
            // "("
            self.walker.goto_first_child();

            // "type"
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::KEYWORD, None)?;
            }

            // $.index
            self.walker.goto_next_sibling();
            {
                let node = self.walker.node();
                let range = crate::util::node::range(&node);
                self.builder.push(range, &lsp::SemanticTokenType::VARIABLE, None)?;
            }

            // skip ")"
            self.walker.goto_parent();

            self.walker.goto_next();

            Ok(())
        }
    }
}
