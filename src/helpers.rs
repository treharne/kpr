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


pub fn split_line(line: String) -> Option<(String, String)> {
    let mut parts = line.splitn(2, ": ");
    
    let timestamp = parts.next()?.trim().parse::<u128>().ok()?;
    let timestamp = format_epoch_time(timestamp);
    let timestamp = timestamp.as_str();
    
    let message = parts.next()?.trim();
    Some((timestamp.to_string(), message.to_string()))
}

pub fn to_line(message: String) -> String {
    let timestamp = now();
    format!("{}: {}", timestamp, message)
}


pub fn format_line_for_output(line: String) -> Option<String> {
    let (timestamp, message) = split_line(line)?;
    Some(format!("{timestamp} {message}"))
}

pub fn format_line_result_for_output(line: Result<String, std::io::Error>) -> Option<String> {
    format_line_option_for_output(line.ok())
}

pub fn format_line_option_for_output<S: Into<String>>(line: Option<S>) -> Option<String> {
    let line = line?.into();
    format_line_for_output(line)
}