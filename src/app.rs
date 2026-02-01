use std::path::PathBuf;
use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tokio::sync::mpsc;

use crate::config::types::Config;
use crate::event::{Event, EventHandler};
use crate::git::service::GitService;
use crate::git::types::GitStatus;
use crate::theme::themes::Theme;
use crate::tui;
use crate::ui;
use crate::watcher::service::FileWatcher;

/// Which section is currently selected
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Staged,
    Unstaged,
}

/// Application actions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    PageUp,
    PageDown,
    TogglePreview,
    ToggleHelp,
    Refresh,
    SwitchSection,
    StageUnstage,
    None,
}

/// Application state
pub struct App {
    /// Path to the Git repository
    pub repo_path: PathBuf,
    /// Current Git status
    pub git_status: Option<GitStatus>,
    /// Current diff content for preview
    pub diff_content: Option<String>,
    /// Currently selected file index within the current section
    pub selected_index: usize,
    /// Currently selected section (staged or unstaged)
    pub selected_section: Section,
    /// Preview scroll offset
    pub preview_scroll: u16,
    /// Whether to show the file preview panel
    pub show_preview: bool,
    /// Whether to show the help menu
    pub show_help: bool,
    /// Application configuration
    pub config: Config,
    /// Current theme
    pub theme: Theme,
    /// Whether watch mode is enabled
    pub watch_mode: bool,
    /// Whether the application should quit
    pub should_quit: bool,
    /// Event sender for async operations
    event_tx: Option<mpsc::UnboundedSender<Event>>,
}

impl App {
    /// Create a new application instance
    pub fn new(repo_path: PathBuf, config: Config, theme: Theme, watch_mode: bool) -> Self {
        Self {
            repo_path,
            git_status: None,
            diff_content: None,
            selected_index: 0,
            selected_section: Section::Unstaged,
            preview_scroll: 0,
            show_preview: config.display.show_file_preview,
            show_help: false,
            config,
            theme,
            watch_mode,
            should_quit: false,
            event_tx: None,
        }
    }

    /// Run the application main loop
    pub async fn run(&mut self) -> Result<()> {
        // Install panic hook
        tui::install_panic_hook();

        // Initialize terminal
        let mut terminal = tui::init()?;

        // Create event handler with tick rate from config
        let tick_rate = Duration::from_millis(self.config.ui.refresh_debounce_ms as u64);
        let mut events = EventHandler::new(tick_rate);
        self.event_tx = Some(events.sender());

        // Verify this is a git repository
        let git_service = GitService::new(self.repo_path.clone());
        if !git_service.is_git_repo().await? {
            tui::restore()?;
            return Err(color_eyre::eyre::eyre!(
                "Not a git repository: {}",
                self.repo_path.display()
            ));
        }

        // Initial git status fetch
        self.refresh_git_status(&git_service).await?;

        // Start file watcher if watch mode is enabled
        let _watcher = if self.watch_mode {
            let watcher = FileWatcher::new(
                self.repo_path.clone(),
                events.sender(),
                Duration::from_millis(self.config.ui.refresh_debounce_ms as u64),
            )?;
            Some(watcher)
        } else {
            None
        };

        // Main event loop
        loop {
            // Render the UI
            terminal.draw(|frame| ui::render(frame, self))?;

            // Handle events
            let event = events.next().await?;
            let action = self.handle_event(event);
            self.handle_action(action, &git_service).await?;

            if self.should_quit {
                break;
            }
        }

        // Restore terminal
        tui::restore()?;

        Ok(())
    }

    /// Handle an incoming event and return the appropriate action
    fn handle_event(&mut self, event: Event) -> Action {
        match event {
            Event::Key(key) => self.handle_key_event(key),
            Event::FileChange => Action::Refresh,
            Event::DiffReady(diff) => {
                self.diff_content = Some(diff);
                Action::None
            }
            Event::Tick | Event::Resize(_, _) => Action::None,
        }
    }

    /// Handle keyboard events
    fn handle_key_event(&self, key: KeyEvent) -> Action {
        // Handle Ctrl+C for quit
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return Action::Quit;
        }

        // Match against configured keybindings
        let kb = &self.config.keybindings;

        match key.code {
            KeyCode::Char(c) => {
                if c.to_string() == kb.quit {
                    Action::Quit
                } else if c.to_string() == kb.refresh {
                    Action::Refresh
                } else if c.to_string() == kb.toggle_preview {
                    Action::TogglePreview
                } else if c.to_string() == kb.help {
                    Action::ToggleHelp
                } else if c.to_string() == kb.stage {
                    Action::StageUnstage
                } else if c == 'j' {
                    Action::MoveDown
                } else if c == 'k' {
                    Action::MoveUp
                } else if c == '\t' {
                    Action::SwitchSection
                } else {
                    Action::None
                }
            }
            KeyCode::Up => {
                if kb.up == "up" {
                    Action::MoveUp
                } else {
                    Action::None
                }
            }
            KeyCode::Down => {
                if kb.down == "down" {
                    Action::MoveDown
                } else {
                    Action::None
                }
            }
            KeyCode::PageUp => {
                if kb.page_up == "pageup" {
                    Action::PageUp
                } else {
                    Action::None
                }
            }
            KeyCode::PageDown => {
                if kb.page_down == "pagedown" {
                    Action::PageDown
                } else {
                    Action::None
                }
            }
            KeyCode::Tab => Action::SwitchSection,
            KeyCode::Esc => {
                if self.show_help {
                    Action::ToggleHelp
                } else {
                    Action::Quit
                }
            }
            _ => Action::None,
        }
    }

    /// Handle an action
    async fn handle_action(&mut self, action: Action, git_service: &GitService) -> Result<()> {
        match action {
            Action::Quit => {
                self.should_quit = true;
            }
            Action::MoveUp => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                    self.preview_scroll = 0;
                    self.update_diff_for_selected();
                }
            }
            Action::MoveDown => {
                let max_index = self.get_current_section_len().saturating_sub(1);
                if self.selected_index < max_index {
                    self.selected_index += 1;
                    self.preview_scroll = 0;
                    self.update_diff_for_selected();
                }
            }
            Action::PageUp => {
                self.selected_index = self.selected_index.saturating_sub(10);
                self.preview_scroll = 0;
                self.update_diff_for_selected();
            }
            Action::PageDown => {
                let max_index = self.get_current_section_len().saturating_sub(1);
                self.selected_index = (self.selected_index + 10).min(max_index);
                self.preview_scroll = 0;
                self.update_diff_for_selected();
            }
            Action::TogglePreview => {
                self.show_preview = !self.show_preview;
            }
            Action::ToggleHelp => {
                self.show_help = !self.show_help;
            }
            Action::Refresh => {
                self.refresh_git_status(git_service).await?;
            }
            Action::SwitchSection => {
                self.switch_section();
            }
            Action::StageUnstage => {
                self.stage_unstage_selected(git_service).await?;
            }
            Action::None => {}
        }
        Ok(())
    }

    /// Refresh the git status
    async fn refresh_git_status(&mut self, git_service: &GitService) -> Result<()> {
        let status = git_service.get_status().await?;
        self.git_status = Some(status);

        // Ensure selected index is valid
        let max_index = self.get_current_section_len().saturating_sub(1);
        if self.selected_index > max_index {
            self.selected_index = max_index;
        }

        // Update diff for selected file
        self.update_diff_for_selected();

        Ok(())
    }

    /// Update the diff content for the currently selected file
    fn update_diff_for_selected(&mut self) {
        if let Some(status) = &self.git_status {
            let files = match self.selected_section {
                Section::Staged => &status.staged_files,
                Section::Unstaged => &status.unstaged_files,
            };

            if let Some(file) = files.get(self.selected_index) {
                let git_service = GitService::new(self.repo_path.clone());
                let path = file.path.clone();
                let staged = file.staged;

                if let Some(tx) = &self.event_tx {
                    let tx = tx.clone();
                    tokio::spawn(async move {
                        if let Ok(diff) = git_service.get_file_diff(&path, staged).await {
                            let _ = tx.send(Event::DiffReady(diff));
                        }
                    });
                }
            } else {
                self.diff_content = None;
            }
        }
    }

    /// Get the number of files in the current section
    fn get_current_section_len(&self) -> usize {
        if let Some(status) = &self.git_status {
            match self.selected_section {
                Section::Staged => status.staged_files.len(),
                Section::Unstaged => status.unstaged_files.len(),
            }
        } else {
            0
        }
    }

    /// Switch between staged and unstaged sections
    fn switch_section(&mut self) {
        if let Some(status) = &self.git_status {
            let (new_section, new_section_len) = match self.selected_section {
                Section::Staged => (Section::Unstaged, status.unstaged_files.len()),
                Section::Unstaged => (Section::Staged, status.staged_files.len()),
            };

            // Only switch if the new section has files
            if new_section_len > 0 {
                self.selected_section = new_section;
                self.selected_index = 0;
                self.preview_scroll = 0;
                self.update_diff_for_selected();
            }
        }
    }

    /// Stage or unstage the currently selected file
    async fn stage_unstage_selected(&mut self, git_service: &GitService) -> Result<()> {
        if let Some(status) = &self.git_status {
            let (file, is_staged) = match self.selected_section {
                Section::Staged => (
                    status.staged_files.get(self.selected_index),
                    true,
                ),
                Section::Unstaged => (
                    status.unstaged_files.get(self.selected_index),
                    false,
                ),
            };

            if let Some(file) = file {
                let path = file.path.clone();

                // Perform the operation
                if is_staged {
                    git_service.unstage_file(&path).await?;
                } else {
                    git_service.stage_file(&path).await?;
                }

                // Refresh the status to reflect the changes
                self.refresh_git_status(git_service).await?;
            }
        }
        Ok(())
    }

    /// Get the currently selected file path
    pub fn get_selected_file_path(&self) -> Option<&str> {
        if let Some(status) = &self.git_status {
            let files = match self.selected_section {
                Section::Staged => &status.staged_files,
                Section::Unstaged => &status.unstaged_files,
            };
            files.get(self.selected_index).map(|f| f.path.as_str())
        } else {
            None
        }
    }
}
