pub fn format_string(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let seconds = secs % 60;
    let minutes = (secs / 60) % 60;
    let hours = (secs / 60) / 60;
    if (hours > 0) & (minutes > 0) & (seconds > 0) {
        return format!("{}h:{}m:{}s", hours, minutes, seconds);
    } else if (minutes > 0) & (seconds > 0) {
        return format!("{}m:{}s", minutes, seconds);
    } else if seconds > 0 {
        return format!("{}s", seconds);
    } else {
        return format!("{}ms", duration.as_millis());
    }
}
