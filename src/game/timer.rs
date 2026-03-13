pub fn format_time(ms: u64) -> String {
    let s = ms / 1000;
    let m = s / 60;
    let ss = s % 60;
    let cs = (ms % 1000) / 10;
    if m > 0 {
        format!("{m}:{ss:02}.{cs:02}")
    } else {
        format!("{ss}.{cs:02}s")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seconds_only() {
        assert_eq!(format_time(5230), "5.23s");
    }

    #[test]
    fn test_with_minutes() {
        assert_eq!(format_time(83450), "1:23.45");
    }

    #[test]
    fn test_zero() {
        assert_eq!(format_time(0), "0.00s");
    }
}
