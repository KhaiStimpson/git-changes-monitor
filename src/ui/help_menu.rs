use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Row, Table},
    Frame,
};

use crate::app::App;

/// Render the help menu overlay
pub fn render_help_menu(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let area = frame.area();

    // Calculate centered popup area
    let popup_width = 50.min(area.width.saturating_sub(4));
    let popup_height = 17.min(area.height.saturating_sub(4));

    let popup_area = centered_rect(popup_width, popup_height, area);

    // Clear the area behind the popup
    frame.render_widget(Clear, popup_area);

    // Create the help content
    let keybindings = vec![
        ("q / Esc", "Quit application"),
        ("r", "Refresh git status"),
        ("p", "Toggle preview panel"),
        ("?", "Toggle help menu"),
        ("↑ / k", "Move selection up"),
        ("↓ / j", "Move selection down"),
        ("PageUp", "Page up"),
        ("PageDown", "Page down"),
        ("Tab", "Switch between staged/unstaged"),
        ("s", "Stage/Unstage selected file"),
        ("Ctrl+C", "Force quit"),
    ];

    let rows: Vec<Row> = keybindings
        .iter()
        .map(|(key, desc)| {
            Row::new(vec![
                Span::styled(
                    *key,
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(*desc, Style::default().fg(theme.text)),
            ])
        })
        .collect();

    let table = Table::new(rows, [Constraint::Length(15), Constraint::Min(20)])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.accent))
                .title(Span::styled(
                    " Keyboard Shortcuts ",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ))
                .title_bottom(Line::from(Span::styled(
                    " Press ? or Esc to close ",
                    Style::default().fg(theme.subtext),
                ))),
        )
        .style(Style::default().bg(theme.overlay));

    frame.render_widget(table, popup_area);
}

/// Helper function to create a centered rectangle
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    Rect::new(x, y, width, height)
}
