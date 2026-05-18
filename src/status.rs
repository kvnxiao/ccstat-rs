use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(unused)]
pub struct StatusInput {
  pub session_id: String,
  pub transcript_path: String,
  pub cwd: String,
  pub model: Model,
  pub workspace: Workspace,
  pub version: String,
  pub output_style: OutputStyle,
  pub cost: Cost,
  pub context_window: ContextWindow,
  #[serde(rename = "exceeds_200k_tokens")]
  pub exceeds_200_k_tokens: bool,
  pub effort: Option<Effort>,
  pub thinking: Option<Thinking>,
  pub rate_limits: Option<RateLimits>,
  pub agent: Option<Agent>,
  pub worktree: Option<Worktree>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(unused)]
pub struct ContextWindow {
  pub total_input_tokens: i64,
  pub total_output_tokens: i64,
  pub context_window_size: i64,
  pub used_percentage: Option<f64>,
  pub remaining_percentage: Option<f64>,
  pub current_usage: Option<CurrentUsage>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentUsage {
  pub input_tokens: i64,
  pub output_tokens: i64,
  pub cache_creation_input_tokens: i64,
  pub cache_read_input_tokens: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(unused)]
pub struct Cost {
  pub total_cost_usd: f64,
  pub total_duration_ms: i64,
  pub total_api_duration_ms: i64,
  pub total_lines_added: i64,
  pub total_lines_removed: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(unused)]
pub struct Model {
  pub id: String,
  pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(unused)]
pub struct OutputStyle {
  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(unused)]
pub struct Workspace {
  pub current_dir: String,
  pub project_dir: String,
  #[serde(default)]
  pub added_dirs: Vec<String>,
  pub git_worktree: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(unused)]
pub struct Effort {
  pub level: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(unused)]
pub struct Thinking {
  pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(unused)]
pub struct RateLimits {
  pub five_hour: Option<RateLimitWindow>,
  pub seven_day: Option<RateLimitWindow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(unused)]
pub struct RateLimitWindow {
  pub used_percentage: f64,
  pub resets_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(unused)]
pub struct Agent {
  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(unused)]
pub struct Worktree {
  pub name: String,
  pub path: String,
  pub branch: Option<String>,
  pub original_cwd: String,
  pub original_branch: Option<String>,
}

impl StatusInput {
  pub fn from_stdin() -> Result<StatusInput> {
    let data: StatusInput = serde_json::from_reader(std::io::stdin())?;
    Ok(data)
  }
}
