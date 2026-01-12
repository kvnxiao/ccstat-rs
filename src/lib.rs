mod status;

use anyhow::Result;

pub fn render_statusline() -> Result<String> {
  // Read and parse StatusInput from stdin
  let data = status::StatusInput::from_stdin()?;
  let model = data.model.display_name;
  let context_window = data.context_window;
  let context_size = context_window.context_window_size;
  let current_dir = data.workspace.current_dir;

  // Calculate current context from current_usage fields
  let current_tokens_usage = if let Some(current_usage) = &context_window.current_usage {
    current_usage.input_tokens
      + current_usage.cache_creation_input_tokens
      + current_usage.cache_read_input_tokens
  } else {
    0
  };

  let percent_used = ((current_tokens_usage * 100) as f64 / context_size as f64).round() as u32;
  let current_tokens_usage_k = current_tokens_usage / 1000;
  let context_size_k = context_size / 1000;

  Ok(format!(
        "✨ {model} | 🧠 {current_tokens_usage_k}K/{context_size_k}K ({percent_used}%)\n🏠 {current_dir}",
    ))
}
