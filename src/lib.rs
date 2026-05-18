mod status;

use anyhow::Result;
use jiff::tz::TimeZone;
use jiff::Timestamp;
use owo_colors::CssColors;
use owo_colors::OwoColorize;
use owo_colors::Style;
use std::fmt::Write;

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

fn effort_style(level: &str) -> Style {
  match level {
    "max" => Style::new().color(CssColors::Red),
    "xhigh" => Style::new().color(CssColors::OrangeRed),
    "high" => Style::new().color(CssColors::Orange),
    "medium" => Style::new().color(CssColors::Gold),
    "low" => Style::new().color(CssColors::Grey),
    _ => Style::new(),
  }
}

fn percent_rgb(percent_used: u32) -> (u8, u8, u8) {
  if percent_used >= 90 {
    (255, 0, 0) // Red
  } else if percent_used >= 80 {
    (255, 69, 0) // OrangeRed
  } else if percent_used >= 60 {
    (255, 165, 0) // Orange
  } else if percent_used >= 40 {
    (255, 215, 0) // Gold
  } else if percent_used >= 20 {
    (255, 255, 224) // LightYellow
  } else {
    (255, 255, 255) // White
  }
}

fn percent_style(percent_used: u32) -> Style {
  let (r, g, b) = percent_rgb(percent_used);
  Style::new().truecolor(r, g, b)
}

fn normalize_path(path: &str) -> String {
  path.replace('\\', "/")
}

fn format_ms_to_min_sec(ms: i64) -> String {
  let total_seconds = ms / 1000;
  let minutes = total_seconds / 60;
  let seconds = total_seconds % 60;

  format!("{}:{:02}", minutes, seconds)
}

fn round_percent(p: f64) -> u32 {
  p.round().clamp(0.0, 100.0) as u32
}

fn format_reset(unix_secs: i64, fmt: &str) -> Option<String> {
  Timestamp::from_second(unix_secs)
    .ok()
    .map(|ts| ts.to_zoned(TimeZone::system()).strftime(fmt).to_string())
}

fn render_rate_window(
  label: &str,
  window: &status::RateLimitWindow,
  reset_fmt: &str,
) -> Result<String> {
  let p = round_percent(window.used_percentage);
  let mut s = format!(
    "{label} {} {}",
    format!("{:>3}%", p).style(percent_style(p)),
    render_bar(p, 10),
  );
  if let Some(reset) = format_reset(window.resets_at, reset_fmt) {
    write!(s, " {}", format!("↻ {}", reset).color(CssColors::Grey))?;
  }
  Ok(s)
}

fn render_bar(percent: u32, width: usize) -> String {
  // Sub-cell partial blocks give 1/8 resolution per cell, so a 1-2% fill
  // remains visible instead of rounding to empty.
  const PARTIALS: [char; 8] = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉'];
  let p = percent.min(100) as usize;
  let eighths = p * width * 8 / 100;
  let full = (eighths / 8).min(width);
  let partial = eighths % 8;

  // Track is the fill color at ~25% intensity (reads like the same hue at low
  // opacity over a black terminal), so the bar stays color-coherent at every
  // fill level.
  let (r, g, b) = percent_rgb(percent);
  let (dr, dg, db) = (r / 4, g / 4, b / 4);
  let fill_style = Style::new().truecolor(r, g, b);
  let track_style = Style::new().truecolor(dr, dg, db);

  let full_blocks: String = "█".repeat(full);
  let has_partial = full < width && partial > 0;
  let empty_count = width.saturating_sub(full + usize::from(has_partial));
  let empty_blocks: String = "█".repeat(empty_count);

  // Partial cell paints the fractional fill char on the dim-track background,
  // so the right side of the cell continues smoothly into the empty track.
  let partial_part = if has_partial {
    format!(
      "{}",
      String::from(PARTIALS[partial]).style(fill_style.on_truecolor(dr, dg, db))
    )
  } else {
    String::new()
  };

  format!(
    "{}{}{}",
    full_blocks.style(fill_style),
    partial_part,
    empty_blocks.style(track_style),
  )
}

pub fn render_statusline() -> Result<String> {
  // Read and parse StatusInput from stdin
  let data = status::StatusInput::from_stdin()?;
  let model = &data.model.display_name;
  let context_window = &data.context_window;
  let context_size = context_window.context_window_size;
  let current_dir = normalize_path(&data.workspace.current_dir);
  let cost = &data.cost;

  // Use Claude Code's pre-calculated values (v2.1.132+): total_input_tokens is
  // current in-context input; used_percentage matches input-only formula.
  let current_tokens_usage = context_window.total_input_tokens;
  let percent_used = context_window
    .used_percentage
    .map(round_percent)
    .unwrap_or(0);

  let separator = "|";
  let sep_g = separator.color(CssColors::Grey).to_string();
  let percent_used_string = format!("{}%", percent_used);
  let current_tokens_usage_k = format!("{}K", current_tokens_usage / 1000);
  let context_size_k = format!("{}K", context_size / 1000);
  let total_duration_s = format_ms_to_min_sec(cost.total_duration_ms);
  let usage_style = percent_style(percent_used);

  // Build the model segment with optional effort level and thinking glyph.
  let mut model_segment = format!("✨ {}", model.style(model_style(&data.model.id)));
  if let Some(effort) = &data.effort {
    write!(
      model_segment,
      "·{}",
      effort.level.style(effort_style(&effort.level))
    )?;
  }
  if data.thinking.as_ref().map(|t| t.enabled).unwrap_or(false) {
    model_segment.push_str(" 💭");
  }

  // Optional agent segment (between model and tokens).
  let mut agent_segment = String::new();
  if let Some(agent) = &data.agent {
    write!(
      agent_segment,
      " {sep_g} 🤖 {}",
      agent.name.color(CssColors::Magenta)
    )?;
  }

  // Optional rate limits line (rendered between header and home line).
  let mut rate_limits_line = String::new();
  if let Some(rl) = &data.rate_limits {
    let mut parts: Vec<String> = Vec::new();
    if let Some(fh) = &rl.five_hour {
      parts.push(render_rate_window("5h", fh, "%H:%M")?);
    }
    if let Some(sd) = &rl.seven_day {
      parts.push(render_rate_window("7d", sd, "%b %d %H:%M")?);
    }
    if !parts.is_empty() {
      write!(
        rate_limits_line,
        "\n📊 {}",
        parts.join(&format!("  {sep_g}  "))
      )?;
    }
  }

  // Second line onward: home dir + worktree, then one line per added dir.
  let mut dir_line = format!("🏠 {}", current_dir.bright_cyan());
  let worktree_name = data
    .worktree
    .as_ref()
    .map(|w| w.name.clone())
    .or_else(|| data.workspace.git_worktree.clone());
  if let Some(name) = worktree_name {
    write!(dir_line, " 🌳 {}", name.color(CssColors::LightGreen))?;
  }
  for added in &data.workspace.added_dirs {
    let added_norm = normalize_path(added);
    if added_norm == current_dir {
      continue;
    }
    write!(dir_line, "\n📂 {}", added_norm.color(CssColors::LightCyan))?;
  }

  Ok(format!(
    "{model_segment}{agent_segment} {sep_g} 🧠 {current_tokens_usage_k}/{context_size_k} ({percent_used}) {sep_g} ⏳ {total_duration_s}{rate_limits_line}\n{dir_line}",
    current_tokens_usage_k = current_tokens_usage_k.style(usage_style),
    context_size_k = context_size_k.color(CssColors::Orange),
    percent_used = percent_used_string.style(usage_style),
    total_duration_s = total_duration_s.color(CssColors::Grey),
  ))
}
