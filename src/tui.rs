use crate::app::App;
use crate::data::types::GamePhase;
use crate::ui::{game, leaderboard, post_game, splash};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::prelude::*;
use std::io::{self, stdout};
use std::time::Duration;

pub fn run(app: &mut App) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal, app);

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    result
}

fn run_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        app.tick();

        terminal.draw(|frame| {
            match app.phase {
                GamePhase::Splash => splash::render(frame, app),
                GamePhase::Playing => game::render(frame, app),
                GamePhase::Won | GamePhase::Lost => post_game::render(frame, app),
            }
        })?;

        if event::poll(Duration::from_millis(33))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // Global quit
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    return Ok(());
                }

                match app.phase {
                    GamePhase::Splash => handle_splash(app, key.code),
                    GamePhase::Playing => handle_playing(app, key.code),
                    GamePhase::Won | GamePhase::Lost => {
                        if handle_post_game(app, key.code) {
                            return Ok(());
                        }
                    }
                }
            }
        }
    }
}

fn handle_splash(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Enter | KeyCode::Char(' ') => {
            // If game already finished today, jump to post-game
            if app.phase == GamePhase::Won || app.phase == GamePhase::Lost {
                return;
            }
            app.start_game();
        }
        KeyCode::Char('q') => std::process::exit(0),
        _ => {}
    }
}

fn handle_playing(app: &mut App, code: KeyCode) {
    if app.editing_free {
        match code {
            KeyCode::Enter => app.toggle_free_edit(),
            KeyCode::Esc => {
                app.editing_free = false;
            }
            KeyCode::Backspace => app.backspace(),
            KeyCode::Char(c) => app.type_char(c),
            _ => {}
        }
        return;
    }

    match code {
        KeyCode::Tab | KeyCode::Right => app.next_param(),
        KeyCode::BackTab | KeyCode::Left => app.prev_param(),
        KeyCode::Up => app.prev_option(),
        KeyCode::Down => app.next_option(),
        KeyCode::Enter => {
            if let Some(param) = app.active_param_key() {
                if param.param_type == crate::data::types::ParamType::Free {
                    app.toggle_free_edit();
                } else {
                    app.submit_guess();
                }
            }
        }
        KeyCode::Char(' ') => app.submit_guess(),
        KeyCode::Char('q') => std::process::exit(0),
        _ => {}
    }
}

fn handle_post_game(app: &mut App, code: KeyCode) -> bool {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => true,
        KeyCode::Char('s') => {
            let text = crate::game::share::build_share_text(
                app.day_num,
                &app.puzzle.difficulty.to_string(),
                &app.guesses,
                app.phase == GamePhase::Won,
                app.final_time,
                app.streak,
                &app.ref_code,
                &app.puzzle.scenario,
            );
            if crate::clipboard::copy_to_clipboard(&text) {
                // Could show a brief "Copied!" message
            }
            false
        }
        KeyCode::Char('r') => {
            // Reset today's game — go back to splash
            app.guesses.clear();
            app.phase = GamePhase::Splash;
            app.final_time = None;
            app.timer_start = None;
            app.elapsed_ms = 0;
            app.active_param = 0;
            app.editing_free = false;
            app.free_input_buffer.clear();
            app.persist();
            false
        }
        _ => false,
    }
}
