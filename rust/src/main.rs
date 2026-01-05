use pi::tools::{default_tools, ToolContext, ToolDefinition};
use pi::{parse_args, Args, ListModels, Mode};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process;

fn print_help() {
    println!(
        "pi (rust) minimal CLI

Usage:
  pi [options] [messages...]

Options:
  --help, -h       Show this help
  --version, -v    Show version
  --print, -p      Print mode (single-shot)
  --mode <mode>    Output mode: text (default), json

Notes:
  Interactive mode and other flags are not implemented yet."
    );
}

fn collect_unsupported_flags(parsed: &Args) -> Vec<&'static str> {
    let mut unsupported = Vec::new();

    if parsed.system_prompt.is_some() {
        unsupported.push("--system-prompt");
    }
    if parsed.append_system_prompt.is_some() {
        unsupported.push("--append-system-prompt");
    }
    if parsed.thinking.is_some() {
        unsupported.push("--thinking");
    }
    if parsed.continue_session {
        unsupported.push("--continue");
    }
    if parsed.resume {
        unsupported.push("--resume");
    }
    if parsed.no_session {
        unsupported.push("--no-session");
    }
    if parsed.session.is_some() {
        unsupported.push("--session");
    }
    if parsed.session_dir.is_some() {
        unsupported.push("--session-dir");
    }
    if parsed.models.is_some() {
        unsupported.push("--models");
    }
    if parsed.tools.is_some() {
        unsupported.push("--tools");
    }
    if parsed.hooks.is_some() {
        unsupported.push("--hook");
    }
    if parsed.custom_tools.is_some() {
        unsupported.push("--tool");
    }
    if parsed.export.is_some() {
        unsupported.push("--export");
    }
    if parsed.no_skills {
        unsupported.push("--no-skills");
    }
    if parsed.skills.is_some() {
        unsupported.push("--skills");
    }
    if parsed.list_models.is_some() {
        match parsed.list_models {
            Some(ListModels::All) => unsupported.push("--list-models"),
            Some(ListModels::Pattern(_)) => unsupported.push("--list-models <search>"),
            None => {}
        }
    }
    if !parsed.file_args.is_empty() {
        unsupported.push("@file");
    }
    if matches!(parsed.mode, Some(Mode::Rpc)) {
        unsupported.push("--mode rpc");
    }

    unsupported
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum AuthCredential {
    ApiKey { key: String },
    Oauth { access: String },
}

type AuthStorageData = HashMap<String, AuthCredential>;

#[derive(Debug, Serialize, Clone)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<AnthropicTool>>,
}

#[derive(Debug, Serialize, Clone)]
struct AnthropicMessage {
    role: String,
    content: Vec<AnthropicContentBlock>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum AnthropicContentBlock {
    Text {
        text: String,
    },
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    ToolResult {
        tool_use_id: String,
        content: Vec<AnthropicToolResultContent>,
        is_error: bool,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum AnthropicToolResultContent {
    Text { text: String },
}

#[derive(Debug, Serialize, Clone)]
struct AnthropicTool {
    name: String,
    description: String,
    input_schema: Value,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContentBlock>,
    stop_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AnthropicErrorResponse {
    error: AnthropicError,
}

#[derive(Debug, Deserialize)]
struct AnthropicError {
    message: String,
}

fn build_anthropic_headers(api_key: &str, use_oauth: bool) -> Result<HeaderMap, String> {
    let mut headers = HeaderMap::new();
    headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
    if use_oauth {
        headers.insert(
            "anthropic-beta",
            HeaderValue::from_static("oauth-2025-04-20"),
        );
        let value = HeaderValue::from_str(&format!("Bearer {api_key}"))
            .map_err(|err| format!("Invalid OAuth token: {err}"))?;
        headers.insert("authorization", value);
    } else {
        let value =
            HeaderValue::from_str(api_key).map_err(|err| format!("Invalid API key: {err}"))?;
        headers.insert("x-api-key", value);
    }
    Ok(headers)
}

fn get_agent_dir() -> Option<PathBuf> {
    if let Ok(dir) = env::var("PI_CODING_AGENT_DIR") {
        if !dir.trim().is_empty() {
            return Some(PathBuf::from(dir));
        }
    }

    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).ok()?;
    Some(PathBuf::from(home).join(".pi").join("agent"))
}

fn get_auth_path() -> Option<PathBuf> {
    get_agent_dir().map(|dir| dir.join("auth.json"))
}

fn read_auth_json() -> Option<AuthCredential> {
    let path = get_auth_path()?;
    let content = std::fs::read_to_string(path).ok()?;
    let data: AuthStorageData = serde_json::from_str(&content).ok()?;
    data.get("anthropic").cloned()
}

fn call_anthropic(
    messages: Vec<AnthropicMessage>,
    model: &str,
    api_key: &str,
    use_oauth: bool,
    tools: &[AnthropicTool],
) -> Result<AnthropicResponse, String> {
    let request = AnthropicRequest {
        model: model.to_string(),
        max_tokens: 1024,
        messages,
        system: if use_oauth {
            Some("You are Claude Code, Anthropic's official CLI for Claude.".to_string())
        } else {
            None
        },
        tools: if tools.is_empty() {
            None
        } else {
            Some(tools.to_vec())
        },
    };

    let headers = build_anthropic_headers(api_key, use_oauth)?;
    let client = Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .headers(headers)
        .json(&request)
        .send()
        .map_err(|err| format!("Request failed: {err}"))?;

    let status = response.status();
    if !status.is_success() {
        let text = response.text().unwrap_or_default();
        if let Ok(error_response) = serde_json::from_str::<AnthropicErrorResponse>(&text) {
            return Err(format!("Anthropic error: {}", error_response.error.message));
        }
        return Err(format!("Anthropic error: {} {}", status.as_u16(), text));
    }

    response
        .json::<AnthropicResponse>()
        .map_err(|err| format!("Failed to parse response: {err}"))
}

fn run_print_mode(
    mode: Mode,
    messages: &[String],
    model: &str,
    api_key_override: Option<&str>,
) -> Result<(), String> {
    let prompt = messages.join("\n");
    if prompt.trim().is_empty() {
        return Err("No messages provided.".to_string());
    }

    let cli_api_key = env::var("ANTHROPIC_API_KEY").ok();
    let oauth_token = env::var("ANTHROPIC_OAUTH_TOKEN").ok();
    let auth_credential = read_auth_json();
    let (api_key, use_oauth) = if let Some(key) = api_key_override {
        (key.to_string(), false)
    } else if let Some(credential) = auth_credential {
        match credential {
            AuthCredential::ApiKey { key } => (key, false),
            AuthCredential::Oauth { access } => (access, true),
        }
    } else if let Some(token) = oauth_token {
        (token, true)
    } else if let Some(key) = cli_api_key {
        (key, false)
    } else {
        return Err(
            "Missing Anthropic credentials. Set ANTHROPIC_OAUTH_TOKEN or ANTHROPIC_API_KEY."
                .to_string(),
        );
    };

    let tool_defs = default_tools();
    let tool_specs = tool_defs
        .iter()
        .map(|tool| AnthropicTool {
            name: tool.name.to_string(),
            description: tool.description.to_string(),
            input_schema: tool.input_schema.clone(),
        })
        .collect::<Vec<_>>();

    let cwd = env::current_dir().map_err(|err| format!("Failed to read cwd: {err}"))?;
    let tool_ctx = ToolContext { cwd };

    let mut conversation = vec![AnthropicMessage {
        role: "user".to_string(),
        content: vec![AnthropicContentBlock::Text { text: prompt }],
    }];

    let mut last_response: Option<AnthropicResponse> = None;
    for _ in 0..10 {
        let response = call_anthropic(
            conversation.clone(),
            model,
            &api_key,
            use_oauth,
            &tool_specs,
        )?;
        let tool_uses = extract_tool_uses(&response.content);

        conversation.push(AnthropicMessage {
            role: "assistant".to_string(),
            content: response.content.clone(),
        });

        if tool_uses.is_empty() {
            last_response = Some(response);
            break;
        }

        let tool_results = tool_uses
            .into_iter()
            .map(|tool_use| execute_tool_use(&tool_use, &tool_defs, &tool_ctx))
            .collect();

        conversation.push(AnthropicMessage {
            role: "user".to_string(),
            content: tool_results,
        });
    }

    let response = last_response.ok_or_else(|| "Tool loop exceeded limit".to_string())?;

    match mode {
        Mode::Text => {
            for block in &response.content {
                match block {
                    AnthropicContentBlock::Text { text } => println!("{text}"),
                    AnthropicContentBlock::ToolUse { .. } => {}
                    AnthropicContentBlock::ToolResult { .. } => {}
                }
            }
        }
        Mode::Json => {
            let payload = json!({
                "role": "assistant",
                "content": response.content,
                "stopReason": response.stop_reason,
            });
            println!("{payload}");
        }
        Mode::Rpc => {
            return Err("RPC mode is not implemented yet.".to_string());
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct ToolUse {
    id: String,
    name: String,
    input: Value,
}

fn extract_tool_uses(blocks: &[AnthropicContentBlock]) -> Vec<ToolUse> {
    let mut uses = Vec::new();
    for block in blocks {
        if let AnthropicContentBlock::ToolUse { id, name, input } = block {
            uses.push(ToolUse {
                id: id.clone(),
                name: name.clone(),
                input: input.clone(),
            });
        }
    }
    uses
}

fn execute_tool_use(
    tool_use: &ToolUse,
    tools: &[ToolDefinition],
    ctx: &ToolContext,
) -> AnthropicContentBlock {
    let tool = tools.iter().find(|tool| tool.name == tool_use.name);
    match tool {
        Some(tool) => match (tool.execute)(&tool_use.input, ctx) {
            Ok(output) => AnthropicContentBlock::ToolResult {
                tool_use_id: tool_use.id.clone(),
                content: vec![AnthropicToolResultContent::Text { text: output }],
                is_error: false,
            },
            Err(message) => AnthropicContentBlock::ToolResult {
                tool_use_id: tool_use.id.clone(),
                content: vec![AnthropicToolResultContent::Text { text: message }],
                is_error: true,
            },
        },
        None => AnthropicContentBlock::ToolResult {
            tool_use_id: tool_use.id.clone(),
            content: vec![AnthropicToolResultContent::Text {
                text: format!("Unknown tool: {}", tool_use.name),
            }],
            is_error: true,
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let parsed = parse_args(&args);

    if parsed.version {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if parsed.help {
        print_help();
        return;
    }

    let unsupported = collect_unsupported_flags(&parsed);
    if !unsupported.is_empty() {
        eprintln!(
            "Error: unsupported flag(s) in rust CLI: {}",
            unsupported.join(", ")
        );
        process::exit(1);
    }

    let is_interactive = !parsed.print && parsed.mode.is_none();
    if is_interactive {
        eprintln!("Error: interactive mode is not implemented yet. Use --print or --mode.");
        process::exit(1);
    }

    let mode = parsed.mode.unwrap_or(Mode::Text);
    if matches!(mode, Mode::Rpc) {
        eprintln!("Error: RPC mode is not implemented yet.");
        process::exit(1);
    }

    let provider = parsed.provider.as_deref().unwrap_or("anthropic");
    if provider != "anthropic" {
        eprintln!("Error: unsupported provider \"{provider}\". Only \"anthropic\" is supported.");
        process::exit(1);
    }
    let model = parsed.model.as_deref().unwrap_or("claude-opus-4-5");

    if let Err(message) = run_print_mode(mode, &parsed.messages, model, parsed.api_key.as_deref()) {
        eprintln!("Error: {message}");
        process::exit(1);
    }
}
