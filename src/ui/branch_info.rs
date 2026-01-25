use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::git::types::GitStatus;
use crate::theme::Theme;

use super::utils::sanitize_text;

/// Render the branch info section
pub fn render_branch_info(frame: &mut Frame, area: Rect, app: &App) {
    let theme = &app.theme;

    if let Some(status) = &app.git_status {
        let content = build_branch_info_content(status, theme);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title(Span::styled(
                " Branch Info ",
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ));

        let paragraph = Paragraph::new(content).block(block);
        frame.render_widget(paragraph, area);
    } else {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title(" Branch Info ");

        let paragraph = Paragraph::new("Loading...")
            .style(Style::default().fg(theme.subtext))
            .block(block);
        frame.render_widget(paragraph, area);
    }
}

/// Build the content for the branch info section
fn build_branch_info_content<'a>(status: &GitStatus, theme: &Theme) -> Vec<Line<'a>> {
    let mut lines = Vec::new();

    // Branch line
    let branch_name = sanitize_text(&status.branch.name);
    let mut branch_spans = vec![
        Span::styled(" Branch: ", Style::default().fg(theme.subtext)),
        Span::styled(
            branch_name,
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    // Add upstream info
    if let Some(upstream) = &status.branch.upstream {
        let upstream_name = sanitize_text(upstream);
        branch_spans.push(Span::styled(
            format!(" -> {}", upstream_name),
            Style::default().fg(theme.subtext),
        ));
    }

    // Add ahead/behind
    if status.branch.ahead > 0 || status.branch.behind > 0 {
        branch_spans.push(Span::raw(" "));
        if status.branch.ahead > 0 {
            branch_spans.push(Span::styled(
                format!("↑{}", status.branch.ahead),
                Style::default().fg(theme.success),
            ));
        }
        if status.branch.behind > 0 {
            if status.branch.ahead > 0 {
                branch_spans.push(Span::raw(" "));
            }
            branch_spans.push(Span::styled(
                format!("↓{}", status.branch.behind),
                Style::default().fg(theme.error),
            ));
        }
    }

    lines.push(Line::from(branch_spans));

    // Last commit line
    if let Some(commit) = &status.last_commit {
        // Sanitize commit subject to remove emojis that cause width issues
        let subject = sanitize_text(&commit.subject);
        let author = sanitize_text(&commit.author);
        let commit_spans = vec![
            Span::styled(" Commit: ", Style::default().fg(theme.subtext)),
            Span::styled(commit.hash.clone(), Style::default().fg(theme.warning)),
            Span::styled(" - ", Style::default().fg(theme.subtext)),
            Span::styled(
                truncate_string(&subject, 50),
                Style::default().fg(theme.text),
            ),
            Span::styled(format!(" ({})", author), Style::default().fg(theme.subtext)),
        ];
        lines.push(Line::from(commit_spans));
    }

    lines
}

/// Truncate a string to a maximum length
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
