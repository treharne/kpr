use chrono::{DateTime, Local};
use crate::{ago, cli::DateFormat, records::Record};

type Timestamp = DateTime<Local>;
type TimestampFormatter = fn(Timestamp) -> String;

pub struct Formatter<MF: Fn(&String) -> String> {
    timestamp_formatter: TimestampFormatter,
    message_formatter: MF,
}

impl<MF: Fn(&String) -> String> Formatter<MF> {
    pub fn new(timestamp_formatter: TimestampFormatter, message_formatter: MF) -> Self {
        Self{timestamp_formatter, message_formatter}
    }

    fn format_timestamp(&self, record: &Record) -> String {
        (self.timestamp_formatter)(record.timestamp)
    }

    fn format_message(&self, record: &Record) -> String {
        (self.message_formatter)(&record.message)
    }

    pub fn format_record(&self, record: &Record) -> (String, String) {
        (self.format_timestamp(&record), self.format_message(&record))
    }

    pub fn format_records(&self, records: &[Record]) -> Vec<(String, String)> {
        records
        .iter()
        .map(|record| self.format_record(record))
        .collect()
    }
}


pub fn get_date_fmt_fn(format: DateFormat) -> TimestampFormatter {
    match format {
        DateFormat::Ago => ago::from_datetime,
        DateFormat::Human => |ts| ts.format("%a %e %b %y %k:%M").to_string(),
        DateFormat::ISO => |ts| ts.format("%Y-%m-%d %H:%M:%S").to_string(),
        DateFormat::Epoch => |ts| ts.timestamp().to_string(),
        DateFormat::EpochMs => |ts| ts.timestamp_millis().to_string(),
    }
}
