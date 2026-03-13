use crate::app::App;
use crate::data::types::ParamType;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let params = &app.puzzle.params;
    let col_count = params.len() as u16;
    if col_count == 0 {
        return;
    }
    let col_width = area.width / col_count;

    for (i, param) in params.iter().enumerate() {
        let x = area.x + (i as u16) * col_width;
        let w = if i as u16 == col_count - 1 {
            area.width - (i as u16) * col_width
        } else {
            col_width
        };
        let cell_area = Rect::new(x, area.y, w, area.height);
        let is_active = i == app.active_param;

        let value = app
            .current_values
            .get(&param.key)
            .cloned()
            .unwrap_or_default();

        let display = if is_active && app.editing_free && param.param_type == ParamType::Free {
            format!("[{}|]", app.free_input_buffer)
        } else if is_active && param.param_type == ParamType::Dropdown {
            format!("[<{}>]", value)
        } else if is_active && param.param_type == ParamType::Free {
            let hint = param.hint.as_deref().unwrap_or("type");
            if value.is_empty() {
                format!("[{}]", hint)
            } else {
                format!("[{}]", value)
            }
        } else if param.param_type == ParamType::Free && value.is_empty() {
            " __".to_string()
        } else {
            format!(" {}", value)
        };

        let style = if is_active {
            Style::default()
                .fg(Color::Rgb(74, 222, 128))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Rgb(200, 200, 200))
        };

        let paragraph = Paragraph::new(display)
            .style(style)
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, cell_area);
    }
}
