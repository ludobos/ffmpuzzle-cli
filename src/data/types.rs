use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HintStatus {
    Correct,
    Close,
    Wrong,
    Higher,
    Lower,
    Empty,
}

impl HintStatus {
    pub fn symbol(&self) -> &str {
        match self {
            Self::Correct => "\u{2713}",
            Self::Close => "~",
            Self::Wrong => "\u{2717}",
            Self::Higher => "\u{2191}",
            Self::Lower => "\u{2193}",
            Self::Empty => " ",
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            Self::Correct => "\u{1f7e9}",
            Self::Close => "\u{1f7e8}",
            _ => "\u{1f7e5}",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    None,
}

impl Direction {
    pub fn arrow(&self) -> &str {
        match self {
            Self::Up => "\u{2191}",
            Self::Down => "\u{2193}",
            Self::None => "",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParamType {
    Dropdown,
    Free,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    Splash,
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Standard,
    Hard,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standard => write!(f, "Standard"),
            Self::Hard => write!(f, "Hard"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    pub key: String,
    pub label: String,
    #[serde(rename = "type")]
    pub param_type: ParamType,
    pub answer: String,
    #[serde(default)]
    pub options: Vec<String>,
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credit {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Puzzle {
    pub params: Vec<Param>,
    pub command: String,
    pub scenario: String,
    pub difficulty: Difficulty,
    pub credit: Option<Credit>,
    #[serde(rename = "funFact")]
    pub fun_fact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuessResult {
    pub key: String,
    pub label: String,
    pub value: String,
    pub status: HintStatus,
    pub direction: Direction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayResult {
    pub day_num: u32,
    pub won: bool,
    pub guesses: u32,
    pub time_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedState {
    pub day_num: u32,
    pub guesses: Vec<Vec<GuessResult>>,
    pub phase: GamePhase,
    pub start_time: u64,
    pub final_time: Option<u64>,
    pub streak: u32,
    pub best_streak: u32,
    pub ref_code: String,
    pub session_token: Option<String>,
    pub history: Vec<DayResult>,
}

pub const MAX_GUESSES: usize = 6;

// API types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PuzzleResponse {
    #[serde(rename = "dayNum")]
    pub day_num: u32,
    pub difficulty: Difficulty,
    pub scenario: String,
    pub params: Vec<ParamInfo>,
    pub credit: Option<Credit>,
    #[serde(rename = "maxGuesses")]
    pub max_guesses: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamInfo {
    pub key: String,
    pub label: String,
    #[serde(rename = "type")]
    pub param_type: ParamType,
    #[serde(default)]
    pub options: Vec<String>,
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuessRequest {
    #[serde(rename = "dayNum")]
    pub day_num: u32,
    #[serde(rename = "guessNum")]
    pub guess_num: u32,
    pub values: std::collections::HashMap<String, String>,
    #[serde(rename = "refCode")]
    pub ref_code: String,
    #[serde(rename = "sessionToken")]
    pub session_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuessResponse {
    pub results: Vec<GuessResult>,
    #[serde(rename = "gameOver")]
    pub game_over: bool,
    pub won: Option<bool>,
    #[serde(rename = "guessesUsed")]
    pub guesses_used: u32,
    #[serde(rename = "sessionToken")]
    pub session_token: String,
    pub reveal: Option<RevealData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevealData {
    pub command: String,
    pub scenario: String,
    #[serde(rename = "funFact")]
    pub fun_fact: String,
    pub credit: Option<Credit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    #[serde(rename = "refCode")]
    pub ref_code: String,
    pub guesses: u32,
    #[serde(rename = "timeMs")]
    pub time_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardResponse {
    pub top: Vec<LeaderboardEntry>,
    pub distribution: std::collections::HashMap<String, u32>,
}
