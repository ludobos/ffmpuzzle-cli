use crate::data::puzzles::{get_day_index, get_puzzle_for_day};
use crate::data::types::*;
use crate::game::hints::{get_direction, get_hint};
use crate::game::streak::compute_streak;
use crate::storage::state::{generate_ref_code, load_state, save_state};
use std::collections::HashMap;
use std::time::Instant;

pub struct App {
    pub phase: GamePhase,
    pub day_num: u32,
    pub puzzle: Puzzle,
    pub guesses: Vec<Vec<GuessResult>>,
    pub current_values: HashMap<String, String>,
    pub active_param: usize,
    pub active_option: HashMap<String, usize>,
    pub timer_start: Option<Instant>,
    pub elapsed_ms: u64,
    pub final_time: Option<u64>,
    pub streak: u32,
    pub best_streak: u32,
    pub ref_code: String,
    pub session_token: Option<String>,
    pub history: Vec<DayResult>,
    pub offline: bool,
    pub free_input_buffer: String,
    pub editing_free: bool,
}

impl App {
    pub fn new(offline: bool) -> Self {
        let (day_idx, puzzle) = get_puzzle_for_day();
        let day_num = day_idx as u32;

        let mut app = Self {
            phase: GamePhase::Splash,
            day_num,
            puzzle,
            guesses: Vec::new(),
            current_values: HashMap::new(),
            active_param: 0,
            active_option: HashMap::new(),
            timer_start: None,
            elapsed_ms: 0,
            final_time: None,
            streak: 0,
            best_streak: 0,
            ref_code: generate_ref_code(),
            session_token: None,
            history: Vec::new(),
            offline,
            free_input_buffer: String::new(),
            editing_free: false,
        };

        // Try to restore state
        if let Some(state) = load_state() {
            if state.day_num == day_num {
                match state.phase {
                    // Finished game: keep stats but reset to splash for replay
                    GamePhase::Won | GamePhase::Lost => {
                        app.phase = GamePhase::Splash;
                    }
                    // In-progress game: resume where we left off
                    _ => {
                        app.guesses = state.guesses;
                        app.phase = state.phase;
                        app.final_time = state.final_time;
                    }
                }
                app.ref_code = state.ref_code;
                app.session_token = state.session_token;
            }
            app.streak = state.streak;
            app.best_streak = state.best_streak;
            app.history = state.history;
        }

        // Initialize dropdown positions
        for param in &app.puzzle.params {
            if param.param_type == ParamType::Dropdown && !param.options.is_empty() {
                app.active_option.insert(param.key.clone(), 0);
                app.current_values
                    .insert(param.key.clone(), param.options[0].clone());
            } else {
                app.current_values.insert(param.key.clone(), String::new());
            }
        }

        app
    }

    pub fn start_game(&mut self) {
        if self.phase == GamePhase::Splash {
            // If we restored a finished game, show post-game
            if self.phase != GamePhase::Playing && !self.guesses.is_empty() {
                return;
            }
            self.phase = GamePhase::Playing;
            self.timer_start = Some(Instant::now());
        }
    }

    pub fn tick(&mut self) {
        if self.phase == GamePhase::Playing {
            if let Some(start) = self.timer_start {
                self.elapsed_ms = start.elapsed().as_millis() as u64;
            }
        }
    }

    pub fn active_param_key(&self) -> Option<&Param> {
        self.puzzle.params.get(self.active_param)
    }

    pub fn next_param(&mut self) {
        if self.editing_free {
            return;
        }
        if self.active_param < self.puzzle.params.len() - 1 {
            self.active_param += 1;
        }
    }

    pub fn prev_param(&mut self) {
        if self.editing_free {
            return;
        }
        if self.active_param > 0 {
            self.active_param -= 1;
        }
    }

    pub fn next_option(&mut self) {
        if self.editing_free {
            return;
        }
        if let Some(param) = self.puzzle.params.get(self.active_param) {
            if param.param_type == ParamType::Dropdown && !param.options.is_empty() {
                let idx = self.active_option.entry(param.key.clone()).or_insert(0);
                *idx = (*idx + 1) % param.options.len();
                self.current_values
                    .insert(param.key.clone(), param.options[*idx].clone());
            }
        }
    }

    pub fn prev_option(&mut self) {
        if self.editing_free {
            return;
        }
        if let Some(param) = self.puzzle.params.get(self.active_param) {
            if param.param_type == ParamType::Dropdown && !param.options.is_empty() {
                let idx = self.active_option.entry(param.key.clone()).or_insert(0);
                if *idx == 0 {
                    *idx = param.options.len() - 1;
                } else {
                    *idx -= 1;
                }
                self.current_values
                    .insert(param.key.clone(), param.options[*idx].clone());
            }
        }
    }

    pub fn toggle_free_edit(&mut self) {
        if let Some(param) = self.puzzle.params.get(self.active_param) {
            if param.param_type == ParamType::Free {
                if self.editing_free {
                    // Commit the buffer
                    self.current_values
                        .insert(param.key.clone(), self.free_input_buffer.clone());
                    self.editing_free = false;
                } else {
                    self.free_input_buffer = self
                        .current_values
                        .get(&param.key)
                        .cloned()
                        .unwrap_or_default();
                    self.editing_free = true;
                }
            }
        }
    }

    pub fn type_char(&mut self, c: char) {
        if self.editing_free {
            self.free_input_buffer.push(c);
        }
    }

    pub fn backspace(&mut self) {
        if self.editing_free {
            self.free_input_buffer.pop();
        }
    }

    pub fn submit_guess(&mut self) {
        if self.phase != GamePhase::Playing {
            return;
        }
        if self.editing_free {
            // Commit free input first
            if let Some(param) = self.puzzle.params.get(self.active_param) {
                self.current_values
                    .insert(param.key.clone(), self.free_input_buffer.clone());
                self.editing_free = false;
            }
        }

        let results: Vec<GuessResult> = self
            .puzzle
            .params
            .iter()
            .map(|param| {
                let value = self
                    .current_values
                    .get(&param.key)
                    .cloned()
                    .unwrap_or_default();
                let status = get_hint(&param.key, &value, &param.answer);
                let direction = get_direction(&param.key, &value, &param.answer);
                GuessResult {
                    key: param.key.clone(),
                    label: param.label.clone(),
                    value,
                    status,
                    direction,
                }
            })
            .collect();

        let all_correct = results.iter().all(|r| r.status == HintStatus::Correct);
        self.guesses.push(results);

        if all_correct {
            self.phase = GamePhase::Won;
            self.final_time = Some(self.elapsed_ms);
            let last_day = self
                .history
                .last()
                .map(|h| h.day_num)
                .unwrap_or(0);
            self.streak = compute_streak(self.streak, self.day_num, last_day);
            if self.streak > self.best_streak {
                self.best_streak = self.streak;
            }
            self.history.push(DayResult {
                day_num: self.day_num,
                won: true,
                guesses: self.guesses.len() as u32,
                time_ms: self.final_time,
            });
        } else if self.guesses.len() >= MAX_GUESSES {
            self.phase = GamePhase::Lost;
            self.final_time = Some(self.elapsed_ms);
            self.streak = 0;
            self.history.push(DayResult {
                day_num: self.day_num,
                won: false,
                guesses: self.guesses.len() as u32,
                time_ms: self.final_time,
            });
        }

        // Reset input for next guess
        self.active_param = 0;

        self.persist();
    }

    pub fn persist(&self) {
        let state = PersistedState {
            day_num: self.day_num,
            guesses: self.guesses.clone(),
            phase: self.phase,
            start_time: self.elapsed_ms,
            final_time: self.final_time,
            streak: self.streak,
            best_streak: self.best_streak,
            ref_code: self.ref_code.clone(),
            session_token: self.session_token.clone(),
            history: self.history.clone(),
        };
        let _ = save_state(&state);
    }

    pub fn guess_number(&self) -> usize {
        self.guesses.len() + 1
    }

    pub fn day_index(&self) -> usize {
        get_day_index()
    }
}
