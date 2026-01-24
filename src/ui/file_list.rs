use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::{App, Section};
use crate::git::types::{FileStatus, FileStatusType};
use crate::theme::Theme;

/// Render the file list section
pub fn render_file_list(frame: &mut Frame, area: Rect, app: &mut App) {
    let theme = &app.theme;

    if let Some(status) = &app.git_status {
        let mut items: Vec<ListItem> = Vec::new();
        let mut current_list_index = 0;
        let mut selected_list_index = None;

        // Staged files section
        if !status.staged_files.is_empty() {
            // Section header
            items.push(ListItem::new(Line::from(vec![Span::styled(
                format!(" STAGED CHANGES ({}) ", status.staged_files.len()),
                Style::default()
                    .fg(theme.staged)
                    .add_modifier(Modifier::BOLD),
            )])));
            current_list_index += 1;

            // Staged files
            for (i, file) in status.staged_files.iter().enumerate() {
                let is_selected =
                    app.selected_section == Section::Staged && app.selected_index == i;
                if is_selected {
                    selected_list_index = Some(current_list_index);
                }
                items.push(create_file_item(file, is_selected, true, theme));
                current_list_index += 1;
            }

            // Empty line separator
            items.push(ListItem::new(Line::from("")));
            current_list_index += 1;
        }

        // Unstaged files section
        if !status.unstaged_files.is_empty() {
            // Section header
            items.push(ListItem::new(Line::from(vec![Span::styled(
                format!(" UNSTAGED CHANGES ({}) ", status.unstaged_files.len()),
                Style::default()
                    .fg(theme.unstaged)
                    .add_modifier(Modifier::BOLD),
            )])));
            current_list_index += 1;

            // Unstaged files
            for (i, file) in status.unstaged_files.iter().enumerate() {
                let is_selected =
                    app.selected_section == Section::Unstaged && app.selected_index == i;
                if is_selected {
                    selected_list_index = Some(current_list_index);
                }
                items.push(create_file_item(file, is_selected, false, theme));
                current_list_index += 1;
            }
        }

        // Handle empty state
        if status.staged_files.is_empty() && status.unstaged_files.is_empty() {
            items.push(ListItem::new(Line::from(vec![Span::styled(
                "  No changes detected",
                Style::default().fg(theme.subtext),
            )])));
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title(Span::styled(
                " Files ",
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ));

        let list = List::new(items).block(block);

        // Create list state for scrolling
        let mut list_state = ListState::default();
        list_state.select(selected_list_index);

        frame.render_stateful_widget(list, area, &mut list_state);
    } else {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title(" Files ");

        let list = List::new(vec![ListItem::new("Loading...")]).block(block);
        frame.render_widget(list, area);
    }
}

/// Create a list item for a file
fn create_file_item<'a>(
    file: &FileStatus,
    is_selected: bool,
    is_staged: bool,
    theme: &Theme,
) -> ListItem<'a> {
    let mut spans = Vec::new();

    // Selection indicator
    if is_selected {
        spans.push(Span::styled(" ▌", Style::default().fg(theme.accent)));
    } else {
        spans.push(Span::raw("  "));
    }

    // Staged/unstaged indicator
    let indicator = if is_staged { "●" } else { "○" };
    let indicator_color = if is_staged {
        theme.staged
    } else {
        theme.unstaged
    };
    spans.push(Span::styled(
        indicator,
        Style::default().fg(indicator_color),
    ));
    spans.push(Span::raw(" "));

    // Status code
    let status_color = get_status_color(file.status, theme);
    spans.push(Span::styled(
        file.status.code(),
        Style::default()
            .fg(status_color)
            .add_modifier(Modifier::BOLD),
    ));
    spans.push(Span::raw(" "));

    // File path
    let path_style = if is_selected {
        Style::default()
            .fg(theme.selection_text)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.text)
    };
    spans.push(Span::styled(file.path.clone(), path_style));

    // Line changes (if any)
    if file.lines_added > 0 || file.lines_deleted > 0 {
        spans.push(Span::raw(" "));
        if file.lines_added > 0 {
            spans.push(Span::styled(
                format!("+{}", file.lines_added),
                Style::default().fg(theme.success),
            ));
        }
        if file.lines_deleted > 0 {
            if file.lines_added > 0 {
                spans.push(Span::raw(" "));
            }
            spans.push(Span::styled(
                format!("-{}", file.lines_deleted),
                Style::default().fg(theme.error),
            ));
        }
    }

    let mut item = ListItem::new(Line::from(spans));

    if is_selected {
        item = item.style(Style::default().bg(theme.selection));
    }

    item
}

/// Get the color for a file status type
fn get_status_color(status: FileStatusType, theme: &Theme) -> ratatui::style::Color {
    match status {
        FileStatusType::Modified => theme.warning,
        FileStatusType::Added => theme.success,
        FileStatusType::Deleted => theme.error,
        FileStatusType::Renamed => theme.info,
        FileStatusType::Copied => theme.info,
        FileStatusType::Untracked => theme.untracked,
        FileStatusType::Unmerged => theme.accent,
    }
}
