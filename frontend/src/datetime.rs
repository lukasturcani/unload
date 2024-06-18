use std::fmt::Display;

use chrono::{DateTime, Local, Utc};

pub fn utc_to_local(time: &DateTime<Utc>) -> DateTime<Local> {
    chrono::DateTime::<chrono::offset::Local>::from_naive_utc_and_offset(
        time.naive_utc(),
        *chrono::offset::Local::now().offset(),
    )
}

pub fn format(time: DateTime<Local>) -> String {
    time.format("%A · %d %B %Y · %I:%M %p").to_string()
}

pub struct TimeDelta {
    negative: bool,
    days: i32,
    hours: i8,
    minutes: i8,
}

impl TimeDelta {
    pub fn is_negative(&self) -> bool {
        self.negative
    }
}

impl Display for TimeDelta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} days {} hours {} minutes",
            self.days, self.hours, self.minutes
        )
    }
}

pub fn time_delta(start: &DateTime<Utc>, stop: &DateTime<Utc>) -> TimeDelta {
    let duration = stop.naive_utc() - start.naive_utc();
    let days = duration.num_days();
    let hours = duration.num_hours() - duration.num_days() * 24;
    let minutes = duration.num_minutes() - (days * 24 * 60) - (hours * 60);
    TimeDelta {
        negative: days < 0 || hours < 0 || minutes < 0,
        days: days.abs() as i32,
        hours: hours.abs() as i8,
        minutes: minutes.abs() as i8,
    }
}
