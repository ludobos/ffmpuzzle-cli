use crate::data::types::{GuessResult, HintStatus};
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

fn status_color(status: HintStatus) -> Color {
    match status {
        HintStatus::Correct => Color::Rgb(61, 214, 140),
        HintStatus::Close => Color::Rgb(229, 184, 0),
        HintStatus::Higher | HintStatus::Lower => Color::Rgb(240, 132, 26),
        HintStatus::Wrong => Color::Rgb(232, 69, 69),
        HintStatus::Empty => Color::Rgb(100, 100, 100),
    }
}

pub fn render_guess_row(frame: &mut Frame, area: Rect, results: &[GuessResult]) {
    let col_count = results.len() as u16;
    if col_count == 0 {
        return;
    }
    let col_width = area.width / col_count;

    for (i, result) in results.iter().enumerate() {
        let x = area.x + (i as u16) * col_width;
        let w = if i as u16 == col_count - 1 {
            area.width - (i as u16) * col_width
        } else {
            col_width
        };
        let cell_area = Rect::new(x, area.y, w, area.height);

        let symbol = result.status.symbol();
        let dir = result.direction.arrow();
        let text = format!("{}{}{}", symbol, result.value, dir);

        let color = status_color(result.status);
        let paragraph = Paragraph::new(text)
            .style(Style::default().fg(color))
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, cell_area);
    }
}

pub fn render_header_row(frame: &mut Frame, area: Rect, labels: &[String]) {
    let col_count = labels.len() as u16;
    if col_count == 0 {
        return;
    }
    let col_width = area.width / col_count;

    for (i, label) in labels.iter().enumerate() {
        let x = area.x + (i as u16) * col_width;
        let w = if i as u16 == col_count - 1 {
            area.width - (i as u16) * col_width
        } else {
            col_width
        };
        let cell_area = Rect::new(x, area.y, w, area.height);

        let short = match label.as_str() {
            "Video Codec" => "Codec",
            "Resolution" => "Res",
            "Container" => "Cont",
            other => other,
        };
        let paragraph = Paragraph::new(short.to_string())
            .style(Style::default().fg(Color::Rgb(150, 150, 150)))
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, cell_area);
    }
}
