use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Render the status bar at the bottom
pub fn render_status_bar(frame: &mut Frame, area: Rect, app: &App) {
    let theme = &app.theme;

    let mut spans = Vec::new();

    // Add leading padding
    spans.push(Span::raw(" "));

    // Keybinding hints
    let keybindings = [
        ("q", "quit"),
        ("r", "refresh"),
        ("p", "preview"),
        ("?", "help"),
        ("↑/k", "up"),
        ("↓/j", "down"),
        ("Tab", "switch"),
    ];

    for (i, (key, action)) in keybindings.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ", Style::default().fg(theme.subtext)));
        }
        spans.push(Span::styled(
            *key,
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            format!(":{}", action),
            Style::default().fg(theme.subtext),
        ));
    }

    // Watch mode indicator
    if app.watch_mode {
        spans.push(Span::styled("  │  ", Style::default().fg(theme.border)));
        spans.push(Span::styled("● ", Style::default().fg(theme.success)));
        spans.push(Span::styled("Watching", Style::default().fg(theme.success)));
    }

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(theme.border));

    let paragraph = Paragraph::new(Line::from(spans)).block(block);
    frame.render_widget(paragraph, area);
}
