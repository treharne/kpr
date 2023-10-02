use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Local, DateTime};

use crate::{tables::make_table, records::Record};


pub fn format_records(records: &[Record], formatter: fn(DateTime<Local>) -> String) -> Vec<(String, String)> {
    records
    .iter()
    .map(|record| (formatter(record.timestamp), record.message.clone()))
    .collect()
}

pub fn format_records_to_table(records: &[Record], formatter: fn(DateTime<Local>) -> String) -> Vec<String> {
    let rows: Vec<(String, String)> = format_records(records, formatter);
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
