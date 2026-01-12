mod status;

use anyhow::Result;
use owo_colors::CssColors;
use owo_colors::OwoColorize;
use owo_colors::Style;

fn model_style(model_id: &str) -> Style {
  let id_lower = model_id.to_lowercase();
  if id_lower.contains("opus") {
    Style::new().bright_blue()
  } else if id_lower.contains("haiku") {
    Style::new().bright_green()
  } else if id_lower.contains("sonnet") {
    Style::new().bright_yellow()
  } else {
    Style::new()
  }
}

fn current_token_usage_style(percent_used: u32) -> Style {
  if percent_used >= 90 {
    Style::new().color(CssColors::Red)
  } else if percent_used >= 80 {
    Style::new().color(CssColors::OrangeRed)
  } else if percent_used >= 60 {
    Style::new().color(CssColors::Orange)
  } else if percent_used >= 40 {
    Style::new().color(CssColors::Gold)
  } else if percent_used >= 20 {
    Style::new().color(CssColors::LightYellow)
  } else {
    Style::new().color(CssColors::White)
  }
}

pub fn render_statusline() -> Result<String> {
  // Read and parse StatusInput from stdin
  let data = status::StatusInput::from_stdin()?;
  let model = &data.model.display_name;
  let context_window = &data.context_window;
  let context_size = &context_window.context_window_size;
  let current_dir = &data.workspace.current_dir;

  // Calculate current context from current_usage fields
  let current_tokens_usage = if let Some(current_usage) = &context_window.current_usage {
    current_usage.input_tokens
      + current_usage.cache_creation_input_tokens
      + current_usage.cache_read_input_tokens
  } else {
    0
  };

  let separator = format!("{}", "|".color(CssColors::Grey));
  let percent_used = ((current_tokens_usage * 100) as f64 / *context_size as f64).round() as u32;
  let percent_used_string = format!("{}%", percent_used);
  let current_tokens_usage_k = format!("{}K", current_tokens_usage / 1000);
  let context_size_k = format!("{}K", context_size / 1000);

  let usage_style = current_token_usage_style(percent_used);

  Ok(format!(
    "✨ {model} {separator} 🧠 {current_tokens_usage_k}/{context_size_k} ({percent_used})\n🏠 {current_dir}",
    model = model.style(model_style(&data.model.id)),
    current_tokens_usage_k = current_tokens_usage_k.style(usage_style),
    context_size_k = context_size_k.color(CssColors::Orange),
    percent_used = percent_used_string.style(usage_style),
    current_dir = current_dir.bright_cyan(),
  ))
}
