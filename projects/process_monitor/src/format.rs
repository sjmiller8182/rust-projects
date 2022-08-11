
const SECONDS_PER_HOURS: u64 = 3600;
const SECONDS_PER_MINUTES: u64 = 60;

pub fn format_seconds(seconds: u64) -> String {
    let hours = seconds / SECONDS_PER_HOURS;
    let seconds = seconds % SECONDS_PER_HOURS;
    let minutes = seconds / SECONDS_PER_MINUTES;
    let seconds = seconds % SECONDS_PER_MINUTES;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn bar(size: u32, utilization: f64) -> String {
    //
    let bar_size = (utilization * size as f64) as usize;
    let bar_str = "=".repeat(bar_size) + ">";
    let spaces = size as usize - bar_size;
    let spaces = " ".repeat(spaces);
    format!("|{}{} | {:.1}/100%", bar_str, spaces, utilization * 100.0)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_format_seconds_1s() {
        let result = format_seconds(1);
        assert_eq!(result, String::from("00:00:01"));
    }

    #[test]
    fn test_format_seconds_1m() {
        let result = format_seconds(60);
        assert_eq!(result, String::from("00:01:00"));
    }

    #[test]
    fn test_format_seconds_1h() {
        let result = format_seconds(3600);
        assert_eq!(result, String::from("01:00:00"));
    }

    #[test]
    fn test_format_seconds_200h() {
        let result = format_seconds(720000);
        assert_eq!(result, String::from("200:00:00"));
    }

    #[test]
    fn test_format_seconds_3h3m3s() {
        let result = format_seconds(10983);
        assert_eq!(result, String::from("03:03:03"));
    }
}