use std::borrow::Cow;

use textwrap::{wrap};
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
use colored::Colorize;

pub fn make_table(rows: &[(String, String)]) -> Vec<String> {
    let timestamp_col_width = max_timestamp_width(rows);
    
    rows
        .into_iter()
        .map(|(timestamp, message)| format_row(timestamp.to_string(), message.to_string(), timestamp_col_width))
        .map(|(timestamp, message)| (timestamp.bright_black().to_string(), message.to_string()))
        .map(|(timestamp, message)| format!("{timestamp}  {message}"))
        .collect()
}


fn format_row(timestamp: String, message: String, timestamp_width: usize) -> (String, String) {
    let timestamp = format!("{timestamp:>timestamp_width$}", timestamp=timestamp, timestamp_width=timestamp_width);

    let message_rows = wrap(&message, 80);
    let message = &message_rows[0];
    if message_rows.len() == 1 {
        return (timestamp, message.to_string());
    }

    let rows = format_row_parts(message_rows, timestamp_width);

    (timestamp, rows)
}


fn format_row_parts(message_rows: Vec<Cow<str>>, timestamp_width: usize) -> String {
    
    let mut parts = message_rows.into_iter();
    let first_part = match parts.next() {
        Some(part) => [part.to_string()].into_iter(),
        None => return String::new(),
    };

    let padding = " ".repeat(timestamp_width + 2);
    let subsequent_parts = parts
        .map(|msg| format!("{padding}{msg}"));

    let all_rows: Vec<_> = first_part.chain(subsequent_parts).collect();

    all_rows.join("\n")
}

fn max_timestamp_width(lines: &[(String, String)]) -> usize {
    lines
        .into_iter()
        .map(|(timestamp, _)| timestamp.graphemes(true).count())
        .max()
        .unwrap_or(0)
}
