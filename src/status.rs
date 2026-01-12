use serde::{Serialize, Deserialize};
use anyhow::Result;

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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(unused)]
pub struct ContextWindow {
    pub total_input_tokens: i64,
    pub total_output_tokens: i64,
    pub context_window_size: i64,
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
}

impl StatusInput {
   pub fn from_stdin() -> Result<StatusInput> {
       let data: StatusInput = serde_json::from_reader(std::io::stdin())?;
       Ok(data)
   }
}
