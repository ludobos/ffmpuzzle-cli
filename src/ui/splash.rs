use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(74, 222, 128)))
        .style(Style::default().bg(Color::Rgb(10, 10, 12)));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "ffmpuzzle",
            Style::default()
                .fg(Color::Rgb(74, 222, 128))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "the daily ffmpeg puzzle",
            Style::default().fg(Color::Rgb(150, 150, 150)),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "guess the encoding parameters",
            Style::default().fg(Color::Rgb(200, 200, 200)),
        )),
        Line::from(Span::styled(
            "in 6 tries or less.",
            Style::default().fg(Color::Rgb(200, 200, 200)),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "$ start",
            Style::default()
                .fg(Color::Rgb(74, 222, 128))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!(
                "streak: {}  |  best: {}",
                app.streak, app.best_streak
            ),
            Style::default().fg(Color::Rgb(150, 150, 150)),
        )),
    ];

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(paragraph, inner);
}
