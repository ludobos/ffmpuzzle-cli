use crate::data::types::{GuessResponse, LeaderboardResponse, PuzzleResponse};
use std::collections::HashMap;

const DEFAULT_BASE_URL: &str = "https://ffmpuzzle.com";

pub struct ApiClient {
    base_url: String,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new(base_url: Option<&str>) -> Self {
        Self {
            base_url: base_url.unwrap_or(DEFAULT_BASE_URL).to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_puzzle(&self) -> Option<PuzzleResponse> {
        let url = format!("{}/api/puzzle", self.base_url);
        self.client
            .get(&url)
            .send()
            .await
            .ok()?
            .json()
            .await
            .ok()
    }

    pub async fn submit_guess(
        &self,
        day_num: u32,
        guess_num: u32,
        values: HashMap<String, String>,
        ref_code: &str,
        session_token: Option<&str>,
    ) -> Option<GuessResponse> {
        let url = format!("{}/api/guess", self.base_url);
        let mut body = serde_json::json!({
            "dayNum": day_num,
            "guessNum": guess_num,
            "values": values,
            "refCode": ref_code,
        });
        if let Some(token) = session_token {
            body["sessionToken"] = serde_json::Value::String(token.to_string());
        }
        self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .ok()?
            .json()
            .await
            .ok()
    }

    pub async fn submit_score(
        &self,
        day_num: u32,
        ref_code: &str,
        guesses: u32,
        time_ms: u64,
        difficulty: &str,
        session_token: &str,
    ) -> bool {
        let url = format!("{}/api/score", self.base_url);
        let body = serde_json::json!({
            "dayNum": day_num,
            "refCode": ref_code,
            "guesses": guesses,
            "timeMs": time_ms,
            "difficulty": difficulty,
            "sessionToken": session_token,
        });
        self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .is_ok()
    }

    pub async fn fetch_leaderboard(&self, day_num: u32) -> Option<LeaderboardResponse> {
        let url = format!("{}/api/leaderboard/{}", self.base_url, day_num);
        self.client
            .get(&url)
            .send()
            .await
            .ok()?
            .json()
            .await
            .ok()
    }
}
