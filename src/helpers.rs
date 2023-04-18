use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{TimeZone, NaiveDateTime, Local};

pub fn now() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    format!("{}", timestamp.as_millis())
}


pub fn format_epoch_time(ms: u128) -> String {
    // takes milliseconds since epoch and returns a formatted string in the local timezone
    let timestamp = NaiveDateTime::from_timestamp_opt((ms / 1000) as i64, 0).unwrap();
    let timestamp = Local.from_utc_datetime(&timestamp);
    timestamp.format("%Y-%m-%d %H:%M:%S").to_string()
}


pub fn split_line(line: &str) -> Option<(String, String)> {
    let mut parts = line.splitn(2, ": ");
    
    let timestamp = parts.next()?.trim().parse::<u128>().ok()?;
    let timestamp = format_epoch_time(timestamp);
    let timestamp = timestamp.as_str();
    
    let message = parts.next()?.trim();
    Some((timestamp.to_string(), message.to_string()))
}

pub fn to_line(message: &str) -> String {
    let timestamp = now();
    format!("{}: {}", timestamp, message)
}


pub fn format_line_for_output(line: &str) -> Option<String> {
    let (timestamp, message) = split_line(line)?;
    Some(format!("{timestamp} {message}"))
}

pub fn format_line_result_for_output(line: Result<String, std::io::Error>) -> Option<String> {
    format_line_option_for_output(line.ok().as_deref())
}

pub fn format_line_option_for_output(line: Option<&str>) -> Option<String> {
    let line = line?.into();
    format_line_for_output(line)
}

pub fn words_from_stdin() -> Result<Vec<String>, std::io::Error> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let message = buffer.trim();
    let message = message
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    Ok(message)
}
