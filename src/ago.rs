use chrono::{Local, DateTime, Duration};

#[allow(non_upper_case_globals)] const secs: fn(i64) -> Duration = Duration::seconds;
#[allow(non_upper_case_globals)] const mins: fn(i64) -> Duration = Duration::minutes;
#[allow(non_upper_case_globals)] const hours: fn(i64) -> Duration = Duration::hours;
#[allow(non_upper_case_globals)] const days: fn(i64) -> Duration = Duration::days;

pub fn from_datetime(timestamp: DateTime<Local>) -> String {
    from_duration(Local::now() - timestamp)
}

fn from_duration(duration: Duration) -> String {
    match duration {
        d if d < secs(0) => "in the future".to_string(),
        d if secs(0) <= d && d < secs(5)        => "just now".to_string(),
        d if secs(5) <= d && d < secs(50)      => seconds_ago(d),
        d if secs(50) <= d && d < secs(90)      => "a minute ago".to_string(),
        d if secs(90) <= d && d < mins(25)      => minutes_ago(d),
        d if mins(25) <= d && d < mins(45)      => "half an hour ago".to_string(),
        d if mins(45) <= d && d < mins(90)      => "an hour ago".to_string(),
        d if mins(90) <= d && d < hours(24)     => hours_ago(d),
        d if hours(24) <= d && d < hours(36)    => "a day ago".to_string(),
        d if hours(36) <= d && d < days(30)     => days_ago(d),
        d if days(30) <= d && d < days(46)      => "a month ago".to_string(),
        d if days(46) <= d && d < days(365)     => months_ago(d),
        d if days(365) <= d && d < days(547)    => "a year ago".to_string(),
        _ => years_ago(duration),
    }
}

fn seconds_ago(delta: Duration) -> String {
    let n_secs = delta.num_seconds();
    format!("{n_secs} seconds ago")
}

fn minutes_ago(delta: Duration) -> String {
    let n_mins = delta.num_seconds() as f32 / 60.0;
    format!("{n_mins:.0} minutes ago")
}

fn hours_ago(delta: Duration) -> String {
    let n_hours = delta.num_minutes() as f32 / 60.0;
    format!("{n_hours:.0} hours ago")
}

fn days_ago(delta: Duration) -> String {
    let n_days = delta.num_hours() as f32 / 24.0;
    format!("{n_days:.0} days ago")
}

fn months_ago(delta: Duration) -> String {
    let n_months = delta.num_days() as f32 / 30.4;
    format!("{n_months:.0} months ago")
}

fn years_ago(delta: Duration) -> String {
    let n_years = delta.num_days() as f32 / 365.0;
    format!("{n_years:.0} years ago")
}

// write tests to check all cases in the format function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {        
        assert_eq!(from_duration(secs(-1)),           "in the future");
        assert_eq!(from_duration(secs(0)),            "just now");
        assert_eq!(from_duration(secs(4)),            "just now");
        assert_eq!(from_duration(secs(5)),            "5 seconds ago");
        assert_eq!(from_duration(secs(19)),           "19 seconds ago");
        assert_eq!(from_duration(secs(20)),           "20 seconds ago");
        assert_eq!(from_duration(secs(49)),           "49 seconds ago");
        assert_eq!(from_duration(secs(50)),           "a minute ago");
        assert_eq!(from_duration(mins(1)),            "a minute ago");
        assert_eq!(from_duration(secs(90)),           "2 minutes ago");
        assert_eq!(from_duration(mins(2)),            "2 minutes ago");
        assert_eq!(from_duration(secs(60*15 - 29)),   "15 minutes ago");
        assert_eq!(from_duration(secs(60*15 + 29)),   "15 minutes ago");
        assert_eq!(from_duration(mins(29)),           "half an hour ago");
        assert_eq!(from_duration(mins(30)),           "half an hour ago");
        assert_eq!(from_duration(mins(44)),           "half an hour ago");
        assert_eq!(from_duration(mins(45)),           "an hour ago");
        assert_eq!(from_duration(hours(1)),           "an hour ago");
        assert_eq!(from_duration(mins(89)),           "an hour ago");
        assert_eq!(from_duration(mins(90)),           "2 hours ago");
        assert_eq!(from_duration(hours(2)),           "2 hours ago");
        assert_eq!(from_duration(mins(120 + 29)),     "2 hours ago");
        assert_eq!(from_duration(hours(23)),          "23 hours ago");
        assert_eq!(from_duration(days(1)),            "a day ago");
        assert_eq!(from_duration(hours(35)),          "a day ago");
        assert_eq!(from_duration(hours(36)),          "2 days ago");
        assert_eq!(from_duration(days(2)),            "2 days ago");
        assert_eq!(from_duration(days(29)),           "29 days ago");
        assert_eq!(from_duration(days(30)),           "a month ago");
        assert_eq!(from_duration(days(45)),           "a month ago");
        assert_eq!(from_duration(days(46)),           "2 months ago");
        assert_eq!(from_duration(days(60)),           "2 months ago");
        assert_eq!(from_duration(days(364)),          "12 months ago");
        assert_eq!(from_duration(days(365)),          "a year ago");
        assert_eq!(from_duration(days(730)),          "2 years ago");
        assert_eq!(from_duration(days(1000)),         "3 years ago");

    }
}