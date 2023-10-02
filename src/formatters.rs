use chrono::{DateTime, Local};
use crate::{ago, cli::DateFormat, records::Record};

type Timestamp = DateTime<Local>;
type TimestampFormatter = fn(Timestamp) -> String;
type MessageFormatter = fn(&str) -> String;

pub struct Formatter {
    timestamp_formatter: TimestampFormatter,
    message_formatter: MessageFormatter,
}

impl Formatter {
    pub fn new(timestamp_formatter: TimestampFormatter, message_formatter: MessageFormatter) -> Self {
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
}


pub fn get_date_fmt_fn(format: DateFormat) -> fn(Timestamp) -> String {
    match format {
        DateFormat::Ago => ago::from_datetime,
        DateFormat::Human => |ts| ts.format("%a %e %b %y %k:%M").to_string(),
        DateFormat::ISO => |ts| ts.format("%Y-%m-%d %H:%M:%S").to_string(),
        DateFormat::Epoch => |ts| ts.timestamp().to_string(),
        DateFormat::EpochMs => |ts| ts.timestamp_millis().to_string(),
    }
}