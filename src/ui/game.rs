use crate::app::App;
use crate::game::timer::format_time;
use crate::ui::{input, result_grid};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let bg = Style::default().bg(Color::Rgb(10, 10, 12));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(50, 50, 50)))
        .style(bg);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Layout: title, scenario, guesses, input, help
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // title + timer
            Constraint::Length(4), // scenario
            Constraint::Min(8),   // guess grid
            Constraint::Length(2), // current input
            Constraint::Length(2), // help
        ])
        .split(inner);

    // Title line
    let title = format!(
        "ffmpuzzle #{} [{}]",
        app.day_num, app.puzzle.difficulty
    );
    let timer = format_time(app.elapsed_ms);
    let title_line = Line::from(vec![
        Span::styled(
            title,
            Style::default()
                .fg(Color::Rgb(74, 222, 128))
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(timer, Style::default().fg(Color::Rgb(200, 200, 200))),
    ]);
    frame.render_widget(Paragraph::new(title_line), chunks[0]);

    // Scenario
    let scenario_text = &app.puzzle.scenario;
    let max_chars = (chunks[1].width as usize).saturating_sub(2) * 3;
    let truncated = if scenario_text.len() > max_chars {
        format!("{}...", &scenario_text[..max_chars.min(scenario_text.len())])
    } else {
        scenario_text.clone()
    };
    let scenario = Paragraph::new(truncated)
        .style(Style::default().fg(Color::Rgb(180, 180, 180)))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(scenario, chunks[1]);

    // Guess grid
    let grid_area = chunks[2];
    let labels: Vec<String> = app.puzzle.params.iter().map(|p| p.label.clone()).collect();

    // Header
    if grid_area.height > 0 {
        let header_area = Rect::new(grid_area.x, grid_area.y, grid_area.width, 1);
        result_grid::render_header_row(frame, header_area, &labels);
    }

    // Previous guesses
    for (i, guess) in app.guesses.iter().enumerate() {
        let y = grid_area.y + 1 + (i as u16) * 2;
        if y + 1 > grid_area.y + grid_area.height {
            break;
        }

        // Separator
        let sep = format!("\u{2500}\u{2500} GUESS #{} ", i + 1);
        let sep_line = Paragraph::new(sep)
            .style(Style::default().fg(Color::Rgb(80, 80, 80)));
        frame.render_widget(sep_line, Rect::new(grid_area.x, y, grid_area.width, 1));

        let row_area = Rect::new(grid_area.x, y + 1, grid_area.width, 1);
        result_grid::render_guess_row(frame, row_area, guess);
    }

    // Current input row
    let guess_label = format!(
        "\u{2500}\u{2500} GUESS #{} (current) ",
        app.guess_number()
    );
    let input_label = Paragraph::new(guess_label)
        .style(Style::default().fg(Color::Rgb(74, 222, 128)));
    let input_area = chunks[3];
    if input_area.height >= 2 {
        frame.render_widget(
            input_label,
            Rect::new(input_area.x, input_area.y, input_area.width, 1),
        );
        input::render(
            frame,
            Rect::new(input_area.x, input_area.y + 1, input_area.width, 1),
            app,
        );
    }

    // Help line
    let help = if app.editing_free {
        "[Enter] valider  [Esc] annuler"
    } else {
        "[Tab] param suivant  [\u{2190}\u{2192}] choix  [Space] soumettre  [Enter] editer  [q] quitter"
    };
    let help_line = Paragraph::new(help)
        .style(Style::default().fg(Color::Rgb(100, 100, 100)))
        .alignment(Alignment::Center);
    frame.render_widget(help_line, chunks[4]);
}
