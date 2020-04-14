//! Elaborator definitions specific to ".wast" files.

use crate::{
    core::{document::Document, language::wast},
    service::elaborator::document_symbol::{self, Data, Work},
    util::node::{symbol_range, SymbolRange},
};
use lsp_types::*;

/// Compute the symbols for a given document.
pub(crate) async fn document_symbol(document: &Document) -> Option<DocumentSymbolResponse> {
    // Vector to collect document symbols into as they are constructed.
    let mut syms: Vec<DocumentSymbol> = vec![];

    // Prepare a filter to discard uninteresting module-child nodes.
    let modulefield_filter = |node: &tree_sitter::Node| {
        [
            *wast::kind::MODULE_FIELD_DATA,
            *wast::kind::MODULE_FIELD_ELEM,
            *wast::kind::MODULE_FIELD_FUNC,
            *wast::kind::MODULE_FIELD_GLOBAL,
            *wast::kind::MODULE_FIELD_MEMORY,
            *wast::kind::MODULE_FIELD_TABLE,
            *wast::kind::MODULE_FIELD_TYPE,
        ]
        .contains(&node.kind_id())
    };

    // Prepare the syntax tree.
    let tree = document.tree.lock().await.clone();
    let node = tree.root_node();

    // Prepare the stack machine:
    //   data: contains data for constructing upcoming DocumentSymbols
    //   work: contains remaining tree_sitter nodes to process
    // FIXME: tune this
    let mut data = vec![];
    let mut work = vec![Work::Node(node)];

    // Convenience macro for processing document symbol nodes.
    macro_rules! push {
        ($node:expr, $empty_name:expr, $kind:expr) => {
            document_symbol::push(
                &document,
                *wast::field::IDENTIFIER,
                &mut data,
                &mut work,
                $node,
                $empty_name,
                $kind,
            )
        };
    }

    // The stack machine work loop.
    while let Some(next) = work.pop() {
        match next {
            // Construct a DocumentSymbol and pop data stack
            Work::Data => {
                if let Some(Data {
                    children_count,
                    kind,
                    name,
                    range,
                    selection_range,
                }) = data.pop()
                {
                    let this = DocumentSymbol {
                        children: if syms.is_empty() {
                            None
                        } else {
                            // Drain the syms array by the number of children nodes we counted for this
                            // DocumentSymbol. This allows us to properly reconstruct symbol nesting.
                            let children = syms.drain(syms.len() - children_count ..);
                            // Process the nodes in reverse (because tree-sitter returns later nodes first).
                            let children = children.rev();
                            Some(children.collect())
                        },
                        deprecated: Default::default(),
                        detail: Default::default(),
                        kind,
                        name: name.to_string(),
                        range,
                        selection_range,
                    };
                    syms.push(this);
                }
            },

            Work::Node(node) if node.kind_id() == *wast::kind::PARSE => {
                let mut cursor = node.walk();
                let commands = node
                    .children_by_field_id(*wast::field::COMMAND, &mut cursor)
                    .map(Work::Node);
                work.extend(commands);
            },

            Work::Node(node) if node.kind_id() == *wast::kind::COMMAND => {
                let command = node.named_child(0).expect("'command' should have a single named child");
                work.push(Work::Node(command));
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE => {
                let SymbolRange {
                    name,
                    range,
                    selection_range,
                } = symbol_range(&document.text.as_bytes(), "<module>", &node, *wast::field::IDENTIFIER);
                work.push(Work::Data);

                let mut children_count = 0;
                for modulefield in node
                    .children_by_field_id(*wast::field::MODULE_FIELD, &mut node.walk())
                    .filter(modulefield_filter)
                {
                    work.push(Work::Node(modulefield));
                    children_count += 1;
                }

                data.push(Data {
                    children_count,
                    kind: SymbolKind::Module,
                    name,
                    range,
                    selection_range,
                });
            },

            Work::Node(node) if node.kind_id() == *wast::kind::INLINE_MODULE => {
                for modulefield in node
                    .children_by_field_id(*wast::field::MODULE_FIELD, &mut node.walk())
                    .filter(modulefield_filter)
                {
                    work.push(Work::Node(modulefield));
                }
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE_FIELD_DATA => {
                push!(&node, "<data>", SymbolKind::Key);
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE_FIELD_ELEM => {
                push!(&node, "<elem>", SymbolKind::Field);
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE_FIELD_FUNC => {
                push!(&node, "<func>", SymbolKind::Function);
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE_FIELD_GLOBAL => {
                push!(&node, "<global>", SymbolKind::Event);
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE_FIELD_MEMORY => {
                push!(&node, "<memory>", SymbolKind::Array);
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE_FIELD_TABLE => {
                push!(&node, "<table>", SymbolKind::Interface);
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE_FIELD_TYPE => {
                push!(&node, "<type>", SymbolKind::TypeParameter);
            },

            _ => {},
        }
    }
    // Collect the syms vec into a new vec in reverse so that document symbols are returned in the
    // correct order. Note that children nodes are reversed _as the symbols are nested_.
    let results = syms.into_iter().rev().collect();

    Some(DocumentSymbolResponse::Nested(results))
}
