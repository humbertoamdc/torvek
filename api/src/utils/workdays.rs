use chrono::{Datelike, Duration, NaiveDate, Weekday};

pub struct Workdays {}

impl Workdays {
    /// Adds N workdays to a given NaiveDate
    pub fn add_workdays(start_date: NaiveDate, workdays: u64) -> NaiveDate {
        let mut date = start_date;
        let mut remaining_days = workdays;

        while remaining_days > 0 {
            date = date + Duration::days(1); // Move to the next day
            if Self::is_workday(date) {
                remaining_days -= 1;
            }
        }

        date
    }

    /// Checks if a date is a workday (not Saturday or Sunday)
    fn is_workday(date: NaiveDate) -> bool {
        let weekday = date.weekday();
        weekday != Weekday::Sat && weekday != Weekday::Sun
    }
}
