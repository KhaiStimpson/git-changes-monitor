use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::theme::Theme;

/// Render the file preview section (diff view)
pub fn render_file_preview(frame: &mut Frame, area: Rect, app: &App) {
    let theme = &app.theme;

    let title = if let Some(path) = app.get_selected_file_path() {
        format!(" Preview: {} ", path)
    } else {
        " Preview ".to_string()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(Span::styled(
            title,
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ));

    if let Some(diff) = &app.diff_content {
        let lines = parse_diff_content(diff, theme);

        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((app.preview_scroll, 0));

        frame.render_widget(paragraph, area);
    } else {
        let paragraph = Paragraph::new("Select a file to view diff")
            .style(Style::default().fg(theme.subtext))
            .block(block);
        frame.render_widget(paragraph, area);
    }
}

/// Parse diff content into styled lines
fn parse_diff_content<'a>(diff: &str, theme: &Theme) -> Vec<Line<'a>> {
    let mut lines = Vec::new();

    for line in diff.lines() {
        let styled_line = if line.starts_with("+++") || line.starts_with("---") {
            // File headers
            Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(theme.info).add_modifier(Modifier::BOLD),
            ))
        } else if line.starts_with("@@") {
            // Hunk headers
            Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(theme.accent),
            ))
        } else if line.starts_with('+') {
            // Added lines
            Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(theme.success),
            ))
        } else if line.starts_with('-') {
            // Removed lines
            Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(theme.error),
            ))
        } else if line.starts_with("diff ") || line.starts_with("index ") {
            // Diff metadata
            Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(theme.subtext),
            ))
        } else {
            // Context lines
            Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(theme.text),
            ))
        };

        lines.push(styled_line);
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "No diff available",
            Style::default().fg(theme.subtext),
        )));
    }

    lines
}
