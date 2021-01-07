var searchIndex = JSON.parse('{\
"wasm_language_server":{"doc":"The WebAssembly language server.","i":[[0,"cli","wasm_language_server","Command-line interface for the WASM language server.",null,null],[5,"cli","wasm_language_server::cli","Invokes the command-line interface for the WASM language …",null,[[]]],[0,"core","wasm_language_server","Core functionality for the WASM language server.",null,null],[3,"Document","wasm_language_server::core","The current state of a document.",null,null],[12,"language","","The language type of the document, e.g., \\\"wasm.wast\\\"",0,null],[12,"content","","The current text content of the document.",0,null],[3,"DocumentEdit","","A description of an edit to a [<code>Document</code>].",null,null],[12,"input_edit","","The input edit structure used for modifying the syntax …",1,null],[12,"start_char_idx","","The starting index (as character offset) of the edit\'s …",1,null],[12,"end_char_idx","","The ending index (as character offset) of the edit\'s …",1,null],[12,"text","","The text of the edit.",1,null],[0,"provider","wasm_language_server","Providers of the WebAssembly language server for LSP …",null,null],[0,"document_symbol","wasm_language_server::provider","Elaborates parse trees into structured data to be cached …",null,null],[0,"wast","wasm_language_server::provider::document_symbol","Document symbol definitions for \\\".wast\\\" files.",null,null],[5,"response","wasm_language_server::provider::document_symbol::wast","Compute \\\"textDocument/documentSymbols\\\" for a given …",null,[[["session",3],["document",3],["arc",3],["documentsymbolparams",3]]]],[0,"wat","wasm_language_server::provider::document_symbol","Document symbol definitions for \\\".wat\\\" files.",null,null],[5,"response","wasm_language_server::provider::document_symbol::wat","Compute \\\"textDocument/documentSymbols\\\" for a given …",null,[[["session",3],["document",3],["arc",3],["documentsymbolparams",3]]]],[0,"hover","wasm_language_server::provider","Provides <code>textDocument/hover</code> functionality.",null,null],[5,"response","wasm_language_server::provider::hover","Compute \\\"textDocument/hover\\\" for a given document.",null,[[["session",3],["arc",3],["hoverparams",3]]]],[0,"semantic_tokens","wasm_language_server::provider","Provides <code>textDocument/semanticTokens/*</code> functionality.",null,null],[0,"wast","wasm_language_server::provider::semantic_tokens","Semantic tokens provider definitions for \\\".wast\\\" files.",null,null],[0,"wat","","Semantic tokens provider definitions for \\\".wat\\\" files.",null,null],[5,"document_symbol","wasm_language_server::provider","Provide response for <code>textDocument/documentSymbols</code>.",null,[[["session",3],["arc",3],["documentsymbolparams",3]]]],[0,"server","wasm_language_server","Definitions for the server instance.",null,null],[3,"Server","wasm_language_server::server","The WASM language server instance.",null,null],[11,"new","","Create a new server.",2,[[["client",3]],["result",6]]],[5,"capabilities","","Compute the server capabilities.",null,[[],["servercapabilities",3]]],[0,"service","wasm_language_server","Services (components) of the WebAssembly language server.",null,null],[11,"from","wasm_language_server::core","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"init","","",0,[[]]],[11,"deref","","",0,[[]]],[11,"deref_mut","","",0,[[]]],[11,"drop","","",0,[[]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"equivalent","","",1,[[]]],[11,"init","","",1,[[]]],[11,"deref","","",1,[[]]],[11,"deref_mut","","",1,[[]]],[11,"drop","","",1,[[]]],[11,"from","wasm_language_server::server","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"vzip","","",2,[[]]],[11,"init","","",2,[[]]],[11,"deref","","",2,[[]]],[11,"deref_mut","","",2,[[]]],[11,"drop","","",2,[[]]],[11,"clone","wasm_language_server::core","",1,[[],["documentedit",3]]],[11,"eq","","",1,[[["documentedit",3]]]],[11,"ne","","",1,[[["documentedit",3]]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"initialize","wasm_language_server::server","",2,[[["initializeparams",3]],[["pin",3],["box",3]]]],[11,"initialized","","",2,[[["initializedparams",3]],[["pin",3],["box",3]]]],[11,"shutdown","","",2,[[],[["pin",3],["box",3]]]],[11,"did_open","","",2,[[["didopentextdocumentparams",3]],[["pin",3],["box",3]]]],[11,"did_change","","",2,[[["didchangetextdocumentparams",3]],[["pin",3],["box",3]]]],[11,"did_close","","",2,[[["didclosetextdocumentparams",3]],[["pin",3],["box",3]]]],[11,"document_symbol","","",2,[[["documentsymbolparams",3]],[["pin",3],["box",3]]]],[11,"hover","","",2,[[["hoverparams",3]],[["pin",3],["box",3]]]],[11,"semantic_tokens_full","","",2,[[["semantictokensparams",3]],[["pin",3],["box",3]]]],[11,"semantic_tokens_range","","",2,[[["semantictokensrangeparams",3]],[["pin",3],["box",3]]]],[11,"new","wasm_language_server::core","Create a new [<code>Document</code>] for the given language id and …",0,[[],["result",6]]],[11,"build_edit","","Build a [<code>DocumentEdit</code>] from an […",0,[[["textdocumentcontentchangeevent",3]],[["documentedit",3],["result",6]]]],[11,"apply_edit","","Modify the given [<code>lsp::Range</code>] in the document.",0,[[["documentedit",3]]]],[11,"range","","Construct a [<code>tree_sitter::Range</code>] from a [<code>DocumentEdit</code>].",1,[[],["range",3]]]],"p":[[3,"Document"],[3,"DocumentEdit"],[3,"Server"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);