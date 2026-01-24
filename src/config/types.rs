use serde::{Deserialize, Serialize};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Display settings
    pub display: DisplayConfig,
    /// UI settings
    pub ui: UIConfig,
    /// Keybindings
    pub keybindings: KeybindingsConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: DisplayConfig::default(),
            ui: UIConfig::default(),
            keybindings: KeybindingsConfig::default(),
        }
    }
}

/// Display configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayConfig {
    /// Show file path and status
    pub show_file_path_and_status: bool,
    /// Show line change counts
    pub show_line_change_counts: bool,
    /// Show staged vs unstaged sections
    pub show_staged_vs_unstaged: bool,
    /// Show file preview panel
    pub show_file_preview: bool,
    /// Show branch information
    pub show_branch_info: bool,
    /// Show last commit information
    pub show_last_commit_info: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_file_path_and_status: true,
            show_line_change_counts: true,
            show_staged_vs_unstaged: true,
            show_file_preview: true,
            show_branch_info: true,
            show_last_commit_info: true,
        }
    }
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UIConfig {
    /// Color scheme name
    pub color_scheme: String,
    /// Refresh debounce in milliseconds
    pub refresh_debounce_ms: u32,
    /// Maximum lines to show in preview
    pub max_preview_lines: u32,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            color_scheme: "catppuccin".to_string(),
            refresh_debounce_ms: 100,
            max_preview_lines: 20,
        }
    }
}

/// Keybindings configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeybindingsConfig {
    /// Quit the application
    pub quit: String,
    /// Refresh git status
    pub refresh: String,
    /// Toggle preview panel
    pub toggle_preview: String,
    /// Show help menu
    pub help: String,
    /// Move selection up
    pub up: String,
    /// Move selection down
    pub down: String,
    /// Page up
    pub page_up: String,
    /// Page down
    pub page_down: String,
}

impl Default for KeybindingsConfig {
    fn default() -> Self {
        Self {
            quit: "q".to_string(),
            refresh: "r".to_string(),
            toggle_preview: "p".to_string(),
            help: "?".to_string(),
            up: "up".to_string(),
            down: "down".to_string(),
            page_up: "pageup".to_string(),
            page_down: "pagedown".to_string(),
        }
    }
}
