use crate::data::types::GuessResult;
use crate::game::timer::format_time;
use rand::Rng;

const NICKNAMES: &[(u32, &[&str])] = &[
    (1, &["first_try --respect", "one_shot_encoder"]),
    (2, &["codec_whisperer", "encode_master"]),
    (3, &["bitrate_surgeon", "param_hunter"]),
    (4, &["just_in_time", "deadline_encoder"]),
    (5, &["close_call", "buffer_underrun"]),
    (6, &["last_chance", "nearly_segfault"]),
];

const SHAME_NICKS: &[&str] = &["segfault", "core_dumped", "broken_pipe", "bus_error"];
const SHAME_MESSAGES: &[&str] = &[
    "segfault",
    "core dumped",
    "broken pipe",
    "bus error",
    "stack overflow",
    "null pointer",
];

fn pick<'a>(arr: &'a [&str]) -> &'a str {
    let mut rng = rand::thread_rng();
    arr[rng.gen_range(0..arr.len())]
}

pub fn get_share_nickname(guess_count: u32) -> &'static str {
    for &(n, nicks) in NICKNAMES {
        if n == guess_count {
            return pick(nicks);
        }
    }
    pick(SHAME_NICKS)
}

pub fn get_shame_message() -> &'static str {
    pick(SHAME_MESSAGES)
}

pub fn build_share_text(
    day_num: u32,
    difficulty: &str,
    guesses: &[Vec<GuessResult>],
    won: bool,
    final_time: Option<u64>,
    streak: u32,
    ref_code: &str,
    scenario: &str,
) -> String {
    let score = if won {
        format!("{}/6", guesses.len())
    } else {
        "X/6".to_string()
    };
    let time = final_time
        .map(|t| format!(" \u{23f1} {}", format_time(t)))
        .unwrap_or_default();
    let grid: String = guesses
        .iter()
        .map(|row| row.iter().map(|c| c.status.emoji()).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    let scenario_line = format!(
        "\"{}...\"",
        scenario.split('.').next().unwrap_or(scenario)
    );
    let streak_line = if streak > 1 {
        format!(" \u{1f525}{streak}")
    } else {
        String::new()
    };

    if won {
        let nick = get_share_nickname(guesses.len() as u32);
        format!(
            "\u{1f3ac} ffmpuzzle #{day_num} [{difficulty}] \u{2014} {score}{time}\n\
             {nick}{streak_line}\n\n\
             {scenario_line}\n\n\
             {grid}\n\n\
             ffmpuzzle.com?ref={ref_code}"
        )
    } else {
        let shame = get_shame_message();
        format!(
            "\u{1f3ac} ffmpuzzle #{day_num} \u{2014} {shame}\n\n\
             {scenario_line}\n\n\
             {grid}\n\n\
             {shame}. don't ask.\n\
             ffmpuzzle.com?ref={ref_code}"
        )
    }
}
