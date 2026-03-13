use crate::data::types::LeaderboardResponse;
use crate::game::timer::format_time;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table};

pub fn render(frame: &mut Frame, area: Rect, data: &LeaderboardResponse) {
    let block = Block::default()
        .title(" Leaderboard ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(74, 222, 128)))
        .style(Style::default().bg(Color::Rgb(10, 10, 12)));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows: Vec<Row> = data
        .top
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let time_str = entry
                .time_ms
                .map(|t| format_time(t))
                .unwrap_or_else(|| "-".into());
            Row::new(vec![
                format!("#{}", i + 1),
                entry.ref_code.clone(),
                format!("{}/6", entry.guesses),
                time_str,
            ])
        })
        .collect();

    let header = Row::new(vec!["Rank", "Player", "Score", "Time"])
        .style(
            Style::default()
                .fg(Color::Rgb(74, 222, 128))
                .add_modifier(Modifier::BOLD),
        );

    let table = Table::new(
        rows,
        [
            Constraint::Length(6),
            Constraint::Length(8),
            Constraint::Length(6),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .style(Style::default().fg(Color::Rgb(200, 200, 200)));

    frame.render_widget(table, inner);
}
