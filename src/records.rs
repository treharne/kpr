use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

pub struct Record {
    pub timestamp: DateTime<Local>,
    pub message: String,
}

impl Record {
    pub fn new(timestamp: DateTime<Local>, message: String) -> Self {
        Record {
            timestamp,
            message,
        }
    }
    
    pub fn create(message: String) -> Self {
        let timestamp = Local::now();
        Record::new(timestamp, message)
    }

    pub fn from_store(line: &str) -> Option<Self> {
        
        if line.trim().is_empty() {
            return None
        }
        
        let mut parts = line.splitn(2, ": ");
    
        let ms_since_epoch = parts.next()?.trim().parse::<u128>().ok()?;
        let timestamp = Self::datetime_from_epoch(ms_since_epoch)?;
        
        let message = parts.next()?.trim().to_string();
        
        Some(Record::new(timestamp, message))
    }

    pub fn to_store(&self) -> String {
        format!("{}: {}", self.timestamp, self.message)
    }

    fn datetime_from_epoch(ms: u128) -> Option<DateTime<Local>> {
        let utc_datetime = NaiveDateTime::from_timestamp_opt((ms / 1000) as i64, 0)?;
        let datetime = Local.from_utc_datetime(&utc_datetime);
        Some(datetime)
    }
}




