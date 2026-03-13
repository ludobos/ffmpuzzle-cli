use crate::data::types::{Direction, HintStatus};
use crate::game::proximity::{
    AUDIO_FAMILIES, CODEC_FAMILIES, CONTAINER_FAMILIES, RESOLUTION_ORDER,
};

pub fn get_hint(key: &str, guess: &str, answer: &str) -> HintStatus {
    if guess.is_empty() {
        return HintStatus::Empty;
    }
    let g = guess.trim();
    let a = answer.trim();
    if g.eq_ignore_ascii_case(a) {
        return HintStatus::Correct;
    }

    match key {
        "codec" => match (CODEC_FAMILIES.get(g), CODEC_FAMILIES.get(a)) {
            (Some(gf), Some(af)) if gf == af => HintStatus::Close,
            _ => HintStatus::Wrong,
        },
        "audio" => match (AUDIO_FAMILIES.get(g), AUDIO_FAMILIES.get(a)) {
            (Some(gf), Some(af)) if gf == af => HintStatus::Close,
            _ => HintStatus::Wrong,
        },
        "container" => match (CONTAINER_FAMILIES.get(g), CONTAINER_FAMILIES.get(a)) {
            (Some(gf), Some(af)) if gf == af => HintStatus::Close,
            _ => HintStatus::Wrong,
        },
        "resolution" => {
            let gi = RESOLUTION_ORDER.iter().position(|&r| r == g);
            let ai = RESOLUTION_ORDER.iter().position(|&r| r == a);
            match (gi, ai) {
                (Some(gi), Some(ai)) => {
                    let diff = (gi as i32 - ai as i32).unsigned_abs();
                    if diff == 1 {
                        HintStatus::Close
                    } else if gi < ai {
                        HintStatus::Higher
                    } else {
                        HintStatus::Lower
                    }
                }
                _ => HintStatus::Wrong,
            }
        }
        "quality" | "preset" => {
            let gn = parse_numeric(g);
            let an = parse_numeric(a);
            match (gn, an) {
                (Some(gn), Some(an)) => {
                    if (gn - an).abs() < f64::EPSILON {
                        HintStatus::Correct
                    } else if (gn - an).abs() <= 3.0 {
                        HintStatus::Close
                    } else if gn < an {
                        HintStatus::Higher
                    } else {
                        HintStatus::Lower
                    }
                }
                _ => HintStatus::Wrong,
            }
        }
        _ => {
            if g == a {
                HintStatus::Correct
            } else {
                HintStatus::Wrong
            }
        }
    }
}

pub fn get_direction(key: &str, guess: &str, answer: &str) -> Direction {
    match key {
        "resolution" => {
            let gi = RESOLUTION_ORDER.iter().position(|&r| r == guess);
            let ai = RESOLUTION_ORDER.iter().position(|&r| r == answer);
            match (gi, ai) {
                (Some(gi), Some(ai)) if gi != ai => {
                    if gi < ai {
                        Direction::Up
                    } else {
                        Direction::Down
                    }
                }
                _ => Direction::None,
            }
        }
        "quality" | "preset" => {
            let gn = parse_numeric(guess);
            let an = parse_numeric(answer);
            match (gn, an) {
                (Some(gn), Some(an)) if (gn - an).abs() > f64::EPSILON => {
                    if gn < an {
                        Direction::Up
                    } else {
                        Direction::Down
                    }
                }
                _ => Direction::None,
            }
        }
        _ => Direction::None,
    }
}

fn parse_numeric(s: &str) -> Option<f64> {
    let s = s.trim();
    // Handle bitrate notation like "5M", "150M"
    if let Some(prefix) = s.strip_suffix('M') {
        return prefix.parse::<f64>().ok();
    }
    if let Some(prefix) = s.strip_suffix('K') {
        return prefix.parse::<f64>().ok().map(|v| v / 1000.0);
    }
    s.parse::<f64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_match() {
        assert_eq!(get_hint("codec", "H.265", "H.265"), HintStatus::Correct);
        assert_eq!(get_hint("quality", "23", "23"), HintStatus::Correct);
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(get_hint("codec", "h.265", "H.265"), HintStatus::Correct);
    }

    #[test]
    fn test_codec_families() {
        assert_eq!(get_hint("codec", "H.264", "H.265"), HintStatus::Close);
        assert_eq!(get_hint("codec", "VP9", "AV1"), HintStatus::Close);
        assert_eq!(get_hint("codec", "H.264", "AV1"), HintStatus::Wrong);
    }

    #[test]
    fn test_audio_families() {
        assert_eq!(get_hint("audio", "AAC", "MP3"), HintStatus::Close);
        assert_eq!(get_hint("audio", "AC-3", "E-AC-3"), HintStatus::Close);
        assert_eq!(get_hint("audio", "AAC", "PCM"), HintStatus::Wrong);
    }

    #[test]
    fn test_container_families() {
        assert_eq!(get_hint("container", "MP4", "MOV"), HintStatus::Close);
        assert_eq!(get_hint("container", "MKV", "WebM"), HintStatus::Close);
        assert_eq!(get_hint("container", "MP4", "TS"), HintStatus::Wrong);
    }

    #[test]
    fn test_resolution_close() {
        assert_eq!(
            get_hint("resolution", "720p", "1080p"),
            HintStatus::Close
        );
        assert_eq!(
            get_hint("resolution", "1440p", "1080p"),
            HintStatus::Close
        );
    }

    #[test]
    fn test_resolution_direction() {
        assert_eq!(
            get_hint("resolution", "480p", "1080p"),
            HintStatus::Higher
        );
        assert_eq!(
            get_hint("resolution", "2160p", "1080p"),
            HintStatus::Lower
        );
    }

    #[test]
    fn test_quality_close() {
        assert_eq!(get_hint("quality", "21", "23"), HintStatus::Close);
        assert_eq!(get_hint("quality", "26", "23"), HintStatus::Close);
    }

    #[test]
    fn test_quality_direction() {
        assert_eq!(get_hint("quality", "10", "23"), HintStatus::Higher);
        assert_eq!(get_hint("quality", "30", "23"), HintStatus::Lower);
    }

    #[test]
    fn test_empty() {
        assert_eq!(get_hint("codec", "", "H.265"), HintStatus::Empty);
    }

    #[test]
    fn test_direction_resolution() {
        assert_eq!(get_direction("resolution", "720p", "1080p"), Direction::Up);
        assert_eq!(
            get_direction("resolution", "2160p", "1080p"),
            Direction::Down
        );
        assert_eq!(
            get_direction("resolution", "1080p", "1080p"),
            Direction::None
        );
    }

    #[test]
    fn test_direction_quality() {
        assert_eq!(get_direction("quality", "18", "23"), Direction::Up);
        assert_eq!(get_direction("quality", "30", "23"), Direction::Down);
    }
}
