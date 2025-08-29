use lsp_server::{Connection, Message, Request, Response};
use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse, Diagnostic,
    DiagnosticSeverity, DidChangeTextDocumentParams, DidOpenTextDocumentParams, InitializeResult,
    MarkupContent, MarkupKind, Range, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind, Url,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

static LATEST_TEXT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
static COMMANDS: Lazy<Vec<(&'static str, &'static str, &'static str)>> = Lazy::new(|| {
    // (name, description, stack_effect)
    vec![
        ("nop", "No operation.", "Pop 0 | Push 0"),
        ("push", "Push a value onto the stack.", "Pop 0 | Push 1"),
        (
            "pop",
            "Pop the top value from the stack.",
            "Pop 1(n) | Push 0",
        ),
        (
            "add",
            "Pop two values, push first + second.",
            "Pop 2 | Push 1",
        ),
        (
            "sub",
            "Pop two values, push first - second.",
            "Pop 2 | Push 1",
        ),
        (
            "mul",
            "Pop two values, push first * second.",
            "Pop 2 | Push 1",
        ),
        (
            "div",
            "Pop two values, push first / second.",
            "Pop 2 | Push 1",
        ),
        (
            "mod",
            "Pop two values, push first % second",
            "Pop 2 | Push 1",
        ),
        (
            "neg",
            "Pop a value, push !value",
            "Pop 1 | Push 1",
        ),
        (
            "dup",
            "Duplicate the top value on the stack.",
            "Pop 1 | Push 2",
        ),
        (
            "print",
            "Pop and print the top value to output.",
            "Pop n | Push 0",
        ),
        (
            "println",
            "Pop and print the top value to output with a newline.",
            "Pop n | Push 0",
        ),
        (
            "read",
            "Read a value from input and push it onto the stack.",
            "Pop 0 | Push 1",
        ),
        ("goto", "Jump to the specified label.", "Pop 0 | Push 0"),
        (
            "br",
            "Pop value, if true jump to label.",
            "Pop 1 | Push 0",
        ),
        (
            "load",
            "Load a ariable and push its value.",
            "Pop 0 | Push 1",
        ),
        (
            "store",
            "Store the top of stack into a variable.",
            "Pop 1 | Push 0",
        ),
        (
            "gt",
            "Pop two values, push first > second.",
            "Pop 2 | Push 1",
        ),
        (
            "lt",
            "Pop two values, push first < second.",
            "Pop 2 | Push 1",
        ),
        (
            "ge",
            "Pop two values, push first >= second.",
            "Pop 2 | Push 1",
        ),
        (
            "le",
            "Pop two values, push first <= second.",
            "Pop 2 | Push 1",
        ),
        (
            "eq",
            "Pop two values, push first == second",
            "Pop 2 | Push 1",
        ),
        (
            "ne",
            "Pop two values, push first != second",
            "Pop 2 | Push 1",
        ),
        ("and", "Pop two values, push first & second.", "Pop 2 | Push 1"),
        ("or", "Pop two values, push first | second.", "Pop 2 | Push 1"),
        ("not", "Pop a value, push !first", "Pop 1 | Push 1"),
        ("xor", "Pop two values, push first ^ second.", "Pop 2 | Push 1"),
        (
            "shl",
            "Pop two values, push first << second.",
            "Pop 2 | Push 1",
        ),
        (
            "shr",
            "Pop two values, push first >> second.",
            "Pop 2 | Push 1",
        ),
        ("convert", "Convert value from type.", "Pop 1 | Push 1"),
        (
            "rotl",
            "Pop two ints, rotate first left by second bits.",
            "Pop 2 | Push 1",
        ),
        (
            "rotr",
            "Pop two ints, rotate first right by second bits.",
            "Pop 2 | Push 1",
        ),
        (
            "clz",
            "Count leading zeros of top-of-stack integer.",
            "Pop 1 | Push 1",
        ),
        (
            "ctz",
            "Count trailing zeros of top-of-stack integer.",
            "Pop 1 | Push 1",
        ),
        (
            "min",
            "Pop two values and push the minimum.",
            "Pop 2 | Push 1",
        ),
        (
            "max",
            "Pop two values and push the maximum.",
            "Pop 2 | Push 1",
        ),
        (
            "abs",
            "Pop a value and push its absolute value.",
            "Pop 1 | Push 1",
        ),
        (
            "sign",
            "Pop a value and push -1/0/1 depending on sign.",
            "Pop 1 | Push 1",
        ),
        (
            "ceil",
            "Pop a float and push its ceiling.",
            "Pop 1 | Push 1",
        ),
        (
            "floor",
            "Pop a float and push its floor.",
            "Pop 1 | Push 1",
        ),
        (
            "trunc",
            "Pop a float and push its truncation toward zero.",
            "Pop 1 | Push 1",
        ),
        (
            "sqrt",
            "Pop a numeric value and push its square root (float).",
            "Pop 1 | Push 1",
        ),
        (
            "pow",
            "Pop two values and push first^second (as float).",
            "Pop 2 | Push 1",
        ),
        (
            "sin",
            "Pop a numeric value and push sin(value).",
            "Pop 1 | Push 1",
        ),
        (
            "cos",
            "Pop a numeric value and push cos(value).",
            "Pop 1 | Push 1",
        ),
        (
            "tan",
            "Pop a numeric value and push tan(value).",
            "Pop 1 | Push 1",
        ),
        (
            "asin",
            "Pop a numeric value and push asin(value).",
            "Pop 1 | Push 1",
        ),
        (
            "acos",
            "Pop a numeric value and push acos(value).",
            "Pop 1 | Push 1",
        ),
        (
            "atan",
            "Pop a numeric value and push atan(value).",
            "Pop 1 | Push 1",
        ),
        (
            "sinh",
            "Pop a numeric value and push sinh(value).",
            "Pop 1 | Push 1",
        ),
        (
            "cosh",
            "Pop a numeric value and push cosh(value).",
            "Pop 1 | Push 1",
        ),
        (
            "tanh",
            "Pop a numeric value and push tanh(value).",
            "Pop 1 | Push 1",
        ),
        (
            "asinh",
            "Pop a numeric value and push asinh(value).",
            "Pop 1 | Push 1",
        ),
        (
            "acosh",
            "Pop a numeric value and push acosh(value).",
            "Pop 1 | Push 1",
        ),
        (
            "atanh",
            "Pop a numeric value and push atanh(value).",
            "Pop 1 | Push 1",
        ),
        (
            "exp",
            "Pop a numeric value and push exp(value).",
            "Pop 1 | Push 1",
        ),
        (
            "log",
            "Pop a numeric value and push natural log(value).",
            "Pop 1 | Push 1",
        ),
        (
            "len",
            "Pop a string and push its length.",
            "Pop 1 | Push 1",
        ),
        (
            "getarg",
            "Pop an index and push the command-line argument at that index, or nil if out of range.",
            "Pop 1 | Push 1",
        ),
        (
            "assert",
            "Assert that the top of stack is true.",
            "Pop 1(2) | Push 0",
        ),
        (
            "error",
            "Raise a runtime error with an error message.",
            "Pop 1 | Push 0",
        ),
        (
            "exit",
            "Exit the program with provided exit code.",
            "Pop 1 | Push 0",
        ),
    ]
});

static SIGNATURES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("store", "store <var>");
    m.insert("load", "load <var>");
    m.insert("goto", "goto <label>");
    m.insert("br", "br <label>");
    m.insert("convert", "convert <type>");
    m
});

fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    eprintln!("Starting stacky LSP server");

    let (connection, io_threads) = Connection::stdio();

    let server_capabilities = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        completion_provider: Some(lsp_types::CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec![" ".to_string()]),
            ..Default::default()
        }),
        hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
        signature_help_provider: Some(lsp_types::SignatureHelpOptions {
            trigger_characters: Some(vec![" ".to_string()]),
            ..Default::default()
        }),
        ..Default::default()
    };

    eprintln!("Started stacky LSP server");

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if req.method == "initialize" {
                    let result = InitializeResult {
                        capabilities: server_capabilities.clone(),
                        server_info: None,
                    };
                    let resp = Response {
                        id: req.id,
                        result: Some(serde_json::to_value(result)?),
                        error: None,
                    };
                    connection.sender.send(Message::Response(resp))?;
                    continue;
                }
                if req.method == "shutdown" {
                    let resp = Response {
                        id: req.id,
                        result: Some(serde_json::Value::Null),
                        error: None,
                    };
                    connection.sender.send(Message::Response(resp))?;
                    return Ok(());
                }
                handle_request(&connection, req)?;
            }
            Message::Response(_) => {}
            Message::Notification(notification) => {
                if notification.method == "initialized" {
                    eprintln!("Initialized stacky LSP server");
                } else {
                    handle_notification(&connection, notification)?;
                }
            }
        }
    }

    io_threads.join()?;

    eprintln!("Shutting down stacky LSP server");
    Ok(())
}

fn handle_request(
    connection: &Connection,
    req: Request,
) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    match req.method.as_str() {
        "textDocument/completion" => {
            let params: CompletionParams = serde_json::from_value(req.params)?;
            let completions = get_completions(&params);
            let result = CompletionResponse::Array(completions);
            let resp = Response {
                id: req.id,
                result: Some(serde_json::to_value(result)?),
                error: None,
            };
            connection.sender.send(Message::Response(resp))?;
        }
        "textDocument/hover" => {
            // Handle hover: params contain textDocument and position
            let params: lsp_types::HoverParams = serde_json::from_value(req.params)?;
            // try to find the token under cursor in the latest text
            let text = LATEST_TEXT.lock().unwrap().clone();
            let pos = params.text_document_position_params.position;
            let line_idx = pos.line as usize;
            let mut hover_result: Option<lsp_types::Hover> = None;
            let lines: Vec<&str> = text.lines().collect();
            if line_idx < lines.len() {
                let l = lines[line_idx];
                // determine cursor column and extract the token under cursor (better than split_whitespace)
                let col = params.text_document_position_params.position.character as usize;
                let col = col.min(l.len());

                // find start of word (search backward for whitespace)
                let start = l[..col]
                    .rfind(|c: char| c.is_whitespace())
                    .map(|p| p + 1)
                    .unwrap_or(0);
                // find end of word (search forward for whitespace)
                let end = l[col..]
                    .find(|c: char| c.is_whitespace())
                    .map(|p| col + p)
                    .unwrap_or(l.len());

                let mut token = l[start..end]
                    .trim_matches(|c: char| !c.is_alphanumeric() && c != '_')
                    .to_string();
                // if token empty, try a fallback: split_whitespace and pick a non-empty
                if token.is_empty() {
                    token = l
                        .split_whitespace()
                        .find(|s| !s.is_empty())
                        .unwrap_or("")
                        .to_string();
                }

                if !token.is_empty() {
                    for (name, description, effect) in COMMANDS.iter() {
                        if token == *name {
                            let display = if let Some(sig) = SIGNATURES.get(name) {
                                sig.to_string()
                            } else {
                                name.to_string()
                            };
                            let md = format!(
                                "```stacky\n{}\n```\n\n{}\n\n---\n\n{}",
                                display, description, effect
                            );
                            hover_result = Some(lsp_types::Hover {
                                contents: lsp_types::HoverContents::Markup(MarkupContent {
                                    kind: MarkupKind::Markdown,
                                    value: md,
                                }),
                                range: None,
                            });
                            break;
                        }
                    }
                }
            }
            let resp = Response {
                id: req.id,
                result: Some(serde_json::to_value(hover_result)?),
                error: None,
            };
            connection.sender.send(Message::Response(resp))?;
        }
        _ => {}
    }
    Ok(())
}

fn handle_notification(
    connection: &Connection,
    notification: lsp_server::Notification,
) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    match notification.method.as_str() {
        "textDocument/didOpen" => {
            let params: DidOpenTextDocumentParams = serde_json::from_value(notification.params)?;
            {
                let mut latest = LATEST_TEXT.lock().unwrap();
                *latest = params.text_document.text.clone();
            }
            validate_document(
                connection,
                &params.text_document.uri,
                &params.text_document.text,
            )?;
        }
        "textDocument/didChange" => {
            let params: DidChangeTextDocumentParams = serde_json::from_value(notification.params)?;
            if let Some(change) = params.content_changes.first() {
                {
                    let mut latest = LATEST_TEXT.lock().unwrap();
                    *latest = change.text.clone();
                }
                validate_document(connection, &params.text_document.uri, &change.text)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_document(
    connection: &Connection,
    uri: &Url,
    text: &str,
) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let mut diagnostics = Vec::new();

    match stacky::Script::from_str(text) {
        Ok(_) => {}
        Err(errors) => {
            let lines: Vec<&str> = text.lines().collect();
            for err in errors.inner() {
                let (start_line, start_char) = if err.pos.line == 0 {
                    (0u32, 0u32)
                } else {
                    (
                        (err.pos.line - 1) as u32,
                        if err.pos.col == 0 {
                            0u32
                        } else {
                            (err.pos.col - 1) as u32
                        },
                    )
                };

                let end_char = lines
                    .get(start_line as usize)
                    .map(|l| l.len() as u32)
                    .unwrap_or(start_char + 1);

                diagnostics.push(Diagnostic {
                    range: Range {
                        start: lsp_types::Position {
                            line: start_line,
                            character: start_char,
                        },
                        end: lsp_types::Position {
                            line: start_line,
                            character: end_char,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some(format!("stacky")),
                    message: err.kind.to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
        }
    };

    let params = lsp_types::PublishDiagnosticsParams {
        uri: uri.clone(),
        diagnostics,
        version: None,
    };

    let notification = lsp_server::Notification {
        method: "textDocument/publishDiagnostics".to_string(),
        params: serde_json::to_value(params)?,
    };

    connection
        .sender
        .send(Message::Notification(notification))?;
    Ok(())
}

fn get_completions(_params: &CompletionParams) -> Vec<CompletionItem> {
    let constants = vec!["true", "false", "nil"];

    let line = _params.text_document_position.position.line as usize;
    let col = _params.text_document_position.position.character as usize;
    let text = LATEST_TEXT.lock().unwrap().clone();

    let mut labels = Vec::new();
    let mut locals = Vec::new();
    for l in text.lines() {
        let t = l.trim();
        if t.ends_with(":") {
            labels.push(t.trim_end_matches(":").to_string());
        }
        if t.starts_with("store ") {
            let name = t[6..].split_whitespace().next().unwrap_or("");
            if !name.is_empty() {
                locals.push(name.to_string());
            }
        }
    }

    let mut items = Vec::new();

    let lines: Vec<&str> = text.lines().collect();
    let is_line_head = if line < lines.len() {
        let linetext = lines[line];
        let prefix = &linetext[..col.min(linetext.len())];

        if prefix.contains(';') {
            return vec![];
        }

        if prefix.trim().is_empty() {
            true
        } else {
            prefix.split_whitespace().count() <= 1 && !prefix.ends_with(' ')
        }
    } else {
        true
    };

    if is_line_head {
        for (name, _description, _effect) in COMMANDS.iter() {
            items.push(CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("command".to_string()),
                documentation: None,
                ..Default::default()
            });
        }
    }

    let mut show_constants = false;
    if line < lines.len() {
        let linetext = lines[line];
        let prefix = &linetext[..col.min(linetext.len())];
        if let Some(prev) = prefix.split_whitespace().last() {
            if prev == "push" {
                show_constants = true;
            }
        }
    }

    if show_constants {
        for name in &constants {
            items.push(CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("constant".to_string()),
                documentation: None,
                ..Default::default()
            });
        }
    }

    // type suggestions for convert
    let type_names = vec!["string", "int", "float", "bool", "nil"];
    if line < lines.len() {
        let linetext = lines[line];
        let prefix = &linetext[..col.min(linetext.len())];
        let parts: Vec<&str> = prefix.split_whitespace().collect();
        if parts.len() >= 1 && parts[0] == "convert" {
            // if cursor is after 'convert' and we are typing args, suggest types
            for t in &type_names {
                items.push(CompletionItem {
                    label: t.to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some("type".to_string()),
                    documentation: None,
                    ..Default::default()
                });
            }
        }
    }

    let mut show_labels = false;
    let mut show_locals = false;
    if line < lines.len() {
        let linetext = lines[line];
        let prefix = &linetext[..col.min(linetext.len())];
        if let Some(prev) = prefix.split_whitespace().last() {
            if prev == "goto" || prev == "br" {
                show_labels = true;
            }
            if prev == "load" || prev == "store" {
                show_locals = true;
            }
        }
    }

    if show_labels {
        for label in &labels {
            items.push(CompletionItem {
                label: label.clone(),
                kind: Some(CompletionItemKind::FIELD),
                detail: Some("label".to_string()),
                documentation: None,
                ..Default::default()
            });
        }
    }

    if show_locals {
        for local in &locals {
            items.push(CompletionItem {
                label: local.clone(),
                kind: Some(CompletionItemKind::VARIABLE),
                detail: Some("variable".to_string()),
                documentation: None,
                ..Default::default()
            });
        }
    }

    items
}
