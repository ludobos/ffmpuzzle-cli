use crate::app::App;
use crate::data::types::GamePhase;
use crate::game::share::{get_shame_message, get_share_nickname};
use crate::game::timer::format_time;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(50, 50, 50)))
        .style(Style::default().bg(Color::Rgb(10, 10, 12)));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // title
            Constraint::Length(2), // stats
            Constraint::Length(3), // command label
            Constraint::Min(4),   // command + fun fact
            Constraint::Length(2), // help
        ])
        .split(inner);

    let won = app.phase == GamePhase::Won;

    // Title
    let title = if won {
        let nick = get_share_nickname(app.guesses.len() as u32);
        format!("$ {nick}")
    } else {
        format!("$ {}", get_shame_message())
    };
    let title_color = if won {
        Color::Rgb(74, 222, 128)
    } else {
        Color::Rgb(232, 69, 69)
    };
    frame.render_widget(
        Paragraph::new(title)
            .style(
                Style::default()
                    .fg(title_color)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center),
        chunks[0],
    );

    // Stats line
    let stats = if won {
        let time = app
            .final_time
            .map(|t| format_time(t))
            .unwrap_or_else(|| "?".to_string());
        format!(
            "{} \u{2014} {}/6    streak: {}",
            time,
            app.guesses.len(),
            app.streak
        )
    } else {
        "don't ask.".to_string()
    };
    frame.render_widget(
        Paragraph::new(stats)
            .style(Style::default().fg(Color::Rgb(200, 200, 200)))
            .alignment(Alignment::Center),
        chunks[1],
    );

    // Command
    frame.render_widget(
        Paragraph::new("COMMAND")
            .style(
                Style::default()
                    .fg(Color::Rgb(150, 150, 150))
                    .add_modifier(Modifier::BOLD),
            ),
        chunks[2],
    );

    let mut cmd_lines = vec![
        Line::from(Span::styled(
            &app.puzzle.command,
            Style::default().fg(Color::Rgb(74, 222, 128)),
        )),
        Line::from(""),
    ];

    if !app.puzzle.fun_fact.is_empty() {
        cmd_lines.push(Line::from(Span::styled(
            "FUN FACT",
            Style::default()
                .fg(Color::Rgb(150, 150, 150))
                .add_modifier(Modifier::BOLD),
        )));
        cmd_lines.push(Line::from(Span::styled(
            &app.puzzle.fun_fact,
            Style::default().fg(Color::Rgb(180, 180, 180)),
        )));
    }
    frame.render_widget(
        Paragraph::new(cmd_lines).wrap(Wrap { trim: true }),
        chunks[3],
    );

    // Help
    let share_cmd = if won { "share --brag" } else { "share --shame" };
    let help = format!("[s] {share_cmd}   [q] quit");
    frame.render_widget(
        Paragraph::new(help)
            .style(Style::default().fg(Color::Rgb(100, 100, 100)))
            .alignment(Alignment::Center),
        chunks[4],
    );
}
