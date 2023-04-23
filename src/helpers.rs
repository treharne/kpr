use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{TimeZone, NaiveDateTime, Local, DateTime};

use crate::{ago, cli::DateFormat, tables::make_table};

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

pub fn get_date_fmt_fn(format: DateFormat) -> fn(DateTime<Local>) -> String {
    match format {
        DateFormat::Ago => ago::from_datetime,
        DateFormat::Human => |ts| ts.format("%a %e %b %y %k:%M").to_string(),
        DateFormat::ISO => |ts| ts.format("%Y-%m-%d %H:%M:%S").to_string(),
        DateFormat::Epoch => |ts| ts.timestamp().to_string(),
        DateFormat::EpochMs => |ts| ts.timestamp_millis().to_string(),
    }
}

pub fn format_lines(lines: Vec<String>, formatter: fn(DateTime<Local>) -> String) -> Vec<String> {
    let rows: Vec<(String, String)> = lines
        .iter()
        .filter_map(|line| split_line(line))
        .map(|(timestamp, message)| (formatter(timestamp), message))
        .collect();

    make_table(&rows)       
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
