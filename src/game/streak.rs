pub fn compute_streak(current_streak: u32, current_day: u32, last_played_day: u32) -> u32 {
    if current_day == last_played_day {
        current_streak
    } else if current_day == last_played_day + 1 {
        current_streak + 1
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_day() {
        assert_eq!(compute_streak(3, 5, 5), 3);
    }

    #[test]
    fn test_consecutive() {
        assert_eq!(compute_streak(3, 6, 5), 4);
    }

    #[test]
    fn test_gap() {
        assert_eq!(compute_streak(3, 8, 5), 1);
    }
}
