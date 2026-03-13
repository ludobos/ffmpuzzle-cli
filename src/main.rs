mod api;
mod app;
mod cli;
mod clipboard;
mod data;
mod game;
mod storage;
mod tui;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    match args.command {
        None => {
            // Interactive TUI mode
            let mut app = app::App::new(args.offline);
            if let Err(e) = tui::run(&mut app) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        Some(Commands::Guess {
            codec,
            crf,
            bitrate,
            resolution,
            audio,
            container,
            preset,
        }) => {
            let mut app = app::App::new(args.offline);
            if app.phase != data::types::GamePhase::Playing {
                app.start_game();
            }

            // Set values from flags
            if let Some(v) = codec {
                app.current_values.insert("codec".into(), v);
            }
            if let Some(v) = crf {
                app.current_values.insert("quality".into(), v);
            }
            if let Some(v) = bitrate {
                app.current_values.insert("quality".into(), v);
            }
            if let Some(v) = resolution {
                app.current_values.insert("resolution".into(), v);
            }
            if let Some(v) = audio {
                app.current_values.insert("audio".into(), v);
            }
            if let Some(v) = container {
                app.current_values.insert("container".into(), v);
            }
            if let Some(v) = preset {
                app.current_values.insert("preset".into(), v);
            }

            app.submit_guess();

            // Print results
            if let Some(last) = app.guesses.last() {
                for r in last {
                    let symbol = r.status.symbol();
                    let dir = r.direction.arrow();
                    println!(" {}{:<12} {}{}", symbol, r.label, r.value, dir);
                }
            }
            println!();
            match app.phase {
                data::types::GamePhase::Won => {
                    let nick = game::share::get_share_nickname(app.guesses.len() as u32);
                    println!("$ {nick}");
                    println!(
                        "{}/6  {}",
                        app.guesses.len(),
                        app.final_time
                            .map(|t| game::timer::format_time(t))
                            .unwrap_or_default()
                    );
                }
                data::types::GamePhase::Lost => {
                    let shame = game::share::get_shame_message();
                    println!("$ {shame}");
                    println!("don't ask.");
                }
                _ => {
                    println!("Guess {}/6", app.guesses.len());
                }
            }
        }
        Some(Commands::Status) => {
            let app = app::App::new(args.offline);
            println!("ffmpuzzle #{} [{}]", app.day_num, app.puzzle.difficulty);
            println!("Phase: {:?}", app.phase);
            println!("Guesses: {}/6", app.guesses.len());
            println!("Streak: {} (best: {})", app.streak, app.best_streak);
        }
        Some(Commands::Share { copy }) => {
            let app = app::App::new(args.offline);
            let won = app.phase == data::types::GamePhase::Won;
            if app.guesses.is_empty() {
                println!("No game to share yet.");
                return;
            }
            let text = game::share::build_share_text(
                app.day_num,
                &app.puzzle.difficulty.to_string(),
                &app.guesses,
                won,
                app.final_time,
                app.streak,
                &app.ref_code,
                &app.puzzle.scenario,
            );
            println!("{text}");
            if copy && clipboard::copy_to_clipboard(&text) {
                println!("\nCopied to clipboard!");
            }
        }
        Some(Commands::Stats) => {
            let app = app::App::new(args.offline);
            let total = app.history.len();
            let wins = app.history.iter().filter(|h| h.won).count();
            let win_rate = if total > 0 {
                (wins as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            println!("ffmpuzzle stats");
            println!("\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}");
            println!("Played:     {total}");
            println!("Won:        {wins}");
            println!("Win rate:   {win_rate:.0}%");
            println!("Streak:     {}", app.streak);
            println!("Best:       {}", app.best_streak);
        }
        Some(Commands::Leaderboard) => {
            println!("Leaderboard requires online mode. Use the TUI for now.");
        }
    }
}
