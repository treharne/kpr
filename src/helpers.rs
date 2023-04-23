use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{TimeZone, NaiveDateTime, Local, DateTime};

use crate::{ago, cli::DateFormat};

pub fn now() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    format!("{}", timestamp.as_millis())
}


fn datetime_from_epoch(ms: u128) -> Option<DateTime<Local>> {
    let utc_datetime = NaiveDateTime::from_timestamp_opt((ms / 1000) as i64, 0)?;
    let datetime = Local.from_utc_datetime(&utc_datetime);
    Some(datetime)
}


pub fn split_line(line: &str) -> Option<(DateTime<Local>, String)> {
    let mut parts = line.splitn(2, ": ");
    
    let ms_since_epoch = parts.next()?.trim().parse::<u128>().ok()?;
    let timestamp = datetime_from_epoch(ms_since_epoch)?;
    
    let message = parts.next()?.trim().to_string();
    
    Some((timestamp, message))
}

pub fn to_line(message: &str) -> String {
    let timestamp = now();
    format!("{timestamp}: {message}")
}

pub fn get_fmt_fn(format: DateFormat) -> fn(DateTime<Local>, &str) -> String {
    match format {
        DateFormat::Ago => line_as_ago,
        DateFormat::ISO => line_as_iso1806,
        DateFormat::Epoch => line_as_epoch,
        DateFormat::EpochMs => line_as_epoch_ms,
        DateFormat::Human => line_as_human,
    }
}

pub fn line_as_ago(timestamp: DateTime<Local>, message: &str) -> String {
    let time_ago = ago::from_datetime(timestamp);
    format!("[ {time_ago} ] {message}")
}

pub fn line_as_iso1806(timestamp: DateTime<Local>, message: &str) -> String {
    let formatted_timestamp = timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
    format!("[ {formatted_timestamp} ] {message}")
}

pub fn line_as_epoch(timestamp: DateTime<Local>, message: &str) -> String {
    let epoch = timestamp.timestamp();
    format!("[ {epoch} ] {message}")
}

pub fn line_as_epoch_ms(timestamp: DateTime<Local>, message: &str) -> String {
    let epoch_ms = timestamp.timestamp_millis();
    format!("[ {epoch_ms} ] {message}")
}

pub fn line_as_human(timestamp: DateTime<Local>, message: &str) -> String {
    let formatted_timestamp = timestamp.format("%a %e %b %y %k:%M").to_string();
    format!("[ {formatted_timestamp} ] {message}")
}

pub fn format_lines(lines: Vec<String>, formatter: fn(DateTime<Local>, &str) -> String) -> Vec<String> {
    lines
        .iter()
        .filter_map(|line| split_line(line))
        .map(|(timestamp, message)| formatter(timestamp, &message))
        .collect()
}


pub fn words_from_stdin() -> Result<Vec<String>, std::io::Error> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let message = buffer.trim();
    let message = message
        .split_whitespace()
        .map(ToString::to_string)
        .collect();
    Ok(message)
}
