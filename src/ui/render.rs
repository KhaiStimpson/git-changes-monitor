use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::App;

use super::branch_info::render_branch_info;
use super::file_list::render_file_list;
use super::file_preview::render_file_preview;
use super::help_menu::render_help_menu;
use super::status_bar::render_status_bar;

/// Main render function that composes all UI components
pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    // Main layout: header, content, footer
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Branch info (2 lines + borders)
            Constraint::Min(10),   // Main content
            Constraint::Length(2), // Status bar
        ])
        .split(area);

    // Render branch info if enabled
    if app.config.display.show_branch_info {
        render_branch_info(frame, main_layout[0], app);
    }

    // Content layout: file list and preview (side by side if preview is shown)
    if app.show_preview && app.config.display.show_file_preview {
        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_layout[1]);

        render_file_list(frame, content_layout[0], app);
        render_file_preview(frame, content_layout[1], app);
    } else {
        render_file_list(frame, main_layout[1], app);
    }

    // Render status bar
    render_status_bar(frame, main_layout[2], app);

    // Render help menu overlay if visible
    if app.show_help {
        render_help_menu(frame, app);
    }
}
