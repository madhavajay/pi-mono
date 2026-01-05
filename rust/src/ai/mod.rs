use crate::core::messages::{
    AssistantMessage, ContentBlock, Cost, ToolResultMessage, Usage, UserContent, UserMessage,
};
use std::cell::Cell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub id: String,
    pub provider: String,
    pub api: String,
    pub max_tokens: usize,
}

#[derive(Clone, Debug)]
pub struct Tool {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    User(UserMessage),
    Assistant(AssistantMessage),
    ToolResult(ToolResultMessage),
}

impl Message {
    pub fn role(&self) -> &str {
        match self {
            Message::User(_) => "user",
            Message::Assistant(_) => "assistant",
            Message::ToolResult(_) => "toolResult",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Context {
    pub system_prompt: Option<String>,
    pub messages: Vec<Message>,
    pub tools: Option<Vec<Tool>>,
}

#[derive(Clone, Debug)]
pub struct StreamOptions {
    pub signal: Option<AbortSignal>,
}

#[derive(Clone, Debug)]
pub struct AbortSignal {
    flag: Rc<Cell<bool>>,
}

impl AbortSignal {
    pub fn is_aborted(&self) -> bool {
        self.flag.get()
    }
}

pub struct AbortController {
    flag: Rc<Cell<bool>>,
}

impl Default for AbortController {
    fn default() -> Self {
        Self::new()
    }
}

impl AbortController {
    pub fn new() -> Self {
        Self {
            flag: Rc::new(Cell::new(false)),
        }
    }

    pub fn abort(&self) {
        self.flag.set(true);
    }

    pub fn signal(&self) -> AbortSignal {
        AbortSignal {
            flag: self.flag.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum AssistantMessageEvent {
    Start {
        partial: AssistantMessage,
    },
    TextDelta {
        delta: String,
        partial: AssistantMessage,
    },
    ThinkingDelta {
        delta: String,
        partial: AssistantMessage,
    },
    Done {
        message: AssistantMessage,
    },
    Error {
        message: AssistantMessage,
    },
}

pub struct AssistantMessageEventStream {
    model: Model,
    signal: Option<AbortSignal>,
    chunks: Vec<String>,
    chunk_index: usize,
    started: bool,
    done_emitted: bool,
    partial_text: String,
    result: AssistantMessage,
}

impl AssistantMessageEventStream {
    pub fn result(&self) -> AssistantMessage {
        self.result.clone()
    }

    fn maybe_abort(&mut self) -> Option<AssistantMessageEvent> {
        if let Some(signal) = &self.signal {
            if signal.is_aborted() {
                let message = assistant_message(
                    &self.model,
                    &self.partial_text,
                    "aborted",
                    Some("Request was aborted"),
                );
                self.result = message.clone();
                self.done_emitted = true;
                return Some(AssistantMessageEvent::Error { message });
            }
        }
        None
    }
}

impl Iterator for AssistantMessageEventStream {
    type Item = AssistantMessageEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done_emitted {
            return None;
        }

        if let Some(event) = self.maybe_abort() {
            return Some(event);
        }

        if !self.started {
            self.started = true;
            let partial = assistant_message(&self.model, "", "streaming", None);
            return Some(AssistantMessageEvent::Start { partial });
        }

        if self.chunk_index < self.chunks.len() {
            let delta = self.chunks[self.chunk_index].clone();
            self.chunk_index += 1;
            self.partial_text.push_str(&delta);
            let partial = assistant_message(&self.model, &self.partial_text, "streaming", None);
            return Some(AssistantMessageEvent::TextDelta { delta, partial });
        }

        self.done_emitted = true;
        Some(AssistantMessageEvent::Done {
            message: self.result.clone(),
        })
    }
}

pub fn get_model(provider: &str, id: &str) -> Model {
    Model {
        id: id.to_string(),
        provider: provider.to_string(),
        api: provider_to_api(provider).to_string(),
        max_tokens: 8192,
    }
}

pub fn stream(
    model: &Model,
    context: &Context,
    options: StreamOptions,
) -> AssistantMessageEventStream {
    let response_text = generate_response_text(context);
    let result = assistant_message(model, &response_text, "stop", None);
    let chunks = chunk_text(&response_text, 12);

    AssistantMessageEventStream {
        model: model.clone(),
        signal: options.signal,
        chunks,
        chunk_index: 0,
        started: false,
        done_emitted: false,
        partial_text: String::new(),
        result,
    }
}

pub fn complete(model: &Model, context: &Context, options: StreamOptions) -> AssistantMessage {
    if let Some(signal) = &options.signal {
        if signal.is_aborted() {
            return assistant_message(model, "", "aborted", Some("Request was aborted"));
        }
    }

    let mut stream = stream(model, context, options);
    for _ in &mut stream {}
    stream.result()
}

fn generate_response_text(context: &Context) -> String {
    let last_user = context
        .messages
        .iter()
        .rev()
        .find_map(|message| match message {
            Message::User(user) => Some(user_content_text(user)),
            _ => None,
        });

    if let Some(text) = last_user {
        let lowered = text.to_lowercase();
        if lowered.contains("please continue") {
            return "Name1, Name2, Name3, Name4, Name5.".to_string();
        }
    }

    "This is a streamed response with enough content to trigger an abort. Name1 Name2 Name3 Name4 Name5 Name6 Name7 Name8 Name9 Name10."
        .to_string()
}

fn chunk_text(text: &str, size: usize) -> Vec<String> {
    if text.is_empty() {
        return vec![];
    }
    text.as_bytes()
        .chunks(size)
        .map(|chunk| String::from_utf8_lossy(chunk).to_string())
        .collect()
}

fn assistant_message(
    model: &Model,
    text: &str,
    stop_reason: &str,
    error_message: Option<&str>,
) -> AssistantMessage {
    AssistantMessage {
        content: vec![ContentBlock::Text {
            text: text.to_string(),
            text_signature: None,
        }],
        api: model.api.clone(),
        provider: model.provider.clone(),
        model: model.id.clone(),
        usage: Usage {
            input: 1,
            output: text.len() as i64,
            cache_read: 0,
            cache_write: 0,
            total_tokens: Some(1 + text.len() as i64),
            cost: Some(Cost {
                input: 0.0,
                output: 0.0,
                cache_read: 0.0,
                cache_write: 0.0,
                total: 0.0,
            }),
        },
        stop_reason: stop_reason.to_string(),
        error_message: error_message.map(|value| value.to_string()),
        timestamp: now_millis(),
    }
}

fn user_content_text(message: &UserMessage) -> String {
    match &message.content {
        UserContent::Text(text) => text.clone(),
        UserContent::Blocks(blocks) => blocks
            .iter()
            .filter_map(|block| match block {
                ContentBlock::Text { text, .. } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<String>>()
            .join("\n"),
    }
}

fn provider_to_api(provider: &str) -> &'static str {
    match provider {
        "anthropic" => "anthropic-messages",
        "openai" => "openai-responses",
        "google" => "google-generative-ai",
        "google-gemini-cli" => "google-gemini-cli",
        "google-vertex" => "google-vertex",
        _ => "openai-completions",
    }
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or(0)
}
