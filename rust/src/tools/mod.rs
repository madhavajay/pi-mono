use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct ToolContext {
    pub cwd: PathBuf,
}

pub struct ToolDefinition {
    pub name: &'static str,
    pub description: &'static str,
    pub input_schema: Value,
    pub execute: fn(&Value, &ToolContext) -> Result<String, String>,
}

pub fn default_tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "read",
            description: "Read the contents of a file.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "Path to the file to read (relative or absolute)" },
                    "offset": { "type": "integer", "description": "Line number to start reading from (1-indexed)" },
                    "limit": { "type": "integer", "description": "Maximum number of lines to read" }
                },
                "required": ["path"],
                "additionalProperties": false
            }),
            execute: read_tool,
        },
        ToolDefinition {
            name: "write",
            description: "Write content to a file, creating it if needed.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "Path to the file to write (relative or absolute)" },
                    "content": { "type": "string", "description": "File contents" }
                },
                "required": ["path", "content"],
                "additionalProperties": false
            }),
            execute: write_tool,
        },
        ToolDefinition {
            name: "edit",
            description: "Replace exact text in a file.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "Path to the file to edit (relative or absolute)" },
                    "oldText": { "type": "string", "description": "Exact text to find and replace" },
                    "newText": { "type": "string", "description": "New text to replace the old text with" }
                },
                "required": ["path", "oldText", "newText"],
                "additionalProperties": false
            }),
            execute: edit_tool,
        },
        ToolDefinition {
            name: "bash",
            description: "Execute a bash command in the current working directory.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "command": { "type": "string", "description": "Bash command to execute" }
                },
                "required": ["command"],
                "additionalProperties": false
            }),
            execute: bash_tool,
        },
    ]
}

fn resolve_path(path: &str, cwd: &Path) -> PathBuf {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        path
    } else {
        cwd.join(path)
    }
}

fn read_tool(args: &Value, ctx: &ToolContext) -> Result<String, String> {
    let path = get_string_arg(args, "path")?;
    let offset = get_i64_arg(args, "offset").map(|value| value.max(1) as usize);
    let limit = get_i64_arg(args, "limit").map(|value| value.max(0) as usize);
    let absolute_path = resolve_path(&path, &ctx.cwd);
    let content = fs::read_to_string(&absolute_path)
        .map_err(|err| format!("Failed to read {}: {}", path, err))?;
    let lines: Vec<&str> = content.split('\n').collect();
    if lines.is_empty() {
        return Ok(String::new());
    }

    let start = offset.unwrap_or(1).saturating_sub(1);
    if start >= lines.len() {
        return Err(format!("Offset {} is beyond end of file", start + 1));
    }
    let end = match limit {
        Some(limit) => (start + limit).min(lines.len()),
        None => lines.len(),
    };
    Ok(lines[start..end].join("\n"))
}

fn write_tool(args: &Value, ctx: &ToolContext) -> Result<String, String> {
    let path = get_string_arg(args, "path")?;
    let content = get_string_arg(args, "content")?;
    let absolute_path = resolve_path(&path, &ctx.cwd);
    if let Some(parent) = absolute_path.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("Failed to create directory: {err}"))?;
    }
    fs::write(&absolute_path, content.as_bytes())
        .map_err(|err| format!("Failed to write {}: {}", path, err))?;
    Ok(format!("Wrote {} bytes to {}", content.len(), path))
}

fn edit_tool(args: &Value, ctx: &ToolContext) -> Result<String, String> {
    let path = get_string_arg(args, "path")?;
    let old_text = get_string_arg(args, "oldText")?;
    let new_text = get_string_arg(args, "newText")?;
    let absolute_path = resolve_path(&path, &ctx.cwd);
    let content = fs::read_to_string(&absolute_path)
        .map_err(|err| format!("Failed to read {}: {}", path, err))?;
    let matches: Vec<usize> = content
        .match_indices(old_text.as_str())
        .map(|(idx, _)| idx)
        .collect();
    if matches.is_empty() {
        return Err(format!(
            "Could not find the exact text in {}. The old text must match exactly.",
            path
        ));
    }
    if matches.len() > 1 {
        return Err(format!(
            "Found {} occurrences of the text in {}. The text must be unique.",
            matches.len(),
            path
        ));
    }
    let index = matches[0];
    let mut updated = String::with_capacity(content.len() - old_text.len() + new_text.len());
    updated.push_str(&content[..index]);
    updated.push_str(new_text.as_str());
    updated.push_str(&content[index + old_text.len()..]);
    if updated == content {
        return Err(format!(
            "No changes made to {}. The replacement produced identical content.",
            path
        ));
    }
    fs::write(&absolute_path, updated.as_bytes())
        .map_err(|err| format!("Failed to write {}: {}", path, err))?;
    Ok(format!("Successfully replaced text in {}.", path))
}

fn bash_tool(args: &Value, ctx: &ToolContext) -> Result<String, String> {
    let command = get_string_arg(args, "command")?;
    let output = Command::new("bash")
        .arg("-lc")
        .arg(command)
        .current_dir(&ctx.cwd)
        .output()
        .map_err(|err| format!("Failed to execute bash: {err}"))?;
    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    if !output.status.success() {
        return Err(format!(
            "Command exited with code {:?}\n{}",
            output.status.code(),
            combined.trim_end()
        ));
    }
    if combined.is_empty() {
        Ok("(no output)".to_string())
    } else {
        Ok(combined)
    }
}

fn get_string_arg(args: &Value, key: &str) -> Result<String, String> {
    args.get(key)
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
        .ok_or_else(|| format!("Missing or invalid \"{}\" argument", key))
}

fn get_i64_arg(args: &Value, key: &str) -> Option<i64> {
    args.get(key).and_then(|value| value.as_i64())
}
