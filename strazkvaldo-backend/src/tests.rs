#[cfg(test)]
mod tests {
    use crate::application_enums::Periodicity;
    use chrono::{Datelike, Months, NaiveDate, TimeDelta, Utc};
    use cron::Schedule;
    use std::ops::{Add, Sub};
    use std::str::FromStr;

    fn from_cron(cron: String) -> NaiveDate {
        Schedule::from_str(cron.as_str())
            .unwrap()
            .upcoming(Utc)
            .next()
            .unwrap()
            .date_naive()
    }

    fn generate_prev_cur_next_occurence(
        periodicity: Periodicity,
        periodicity_unit: u32,
    ) -> (NaiveDate, NaiveDate, NaiveDate) {
        let next_occurence = match periodicity {
            Periodicity::Day => from_cron("0 0 0 * * * *".to_string()), // periodicity_unit is irrelevant
            Periodicity::Week => from_cron(format!("0 0 0 * * {periodicity_unit} *")), // periodicity_unit is day_of_week 0 = sunday, 6=saturday
            Periodicity::Month => from_cron(format!("0 0 0 {periodicity_unit} * * *")), // periodicity_unit is day_of_month 0 = sunday, 6=saturday
            Periodicity::Year => NaiveDate::from_ymd_opt(Utc::now().year(), 1, 1)
                .unwrap()
                .add(TimeDelta::days(periodicity_unit as i64)), // periodicity_unit is day_of_year
        };
        let (prev_occurence, todo_occurence) = match periodicity {
            Periodicity::Day => (
                next_occurence.sub(TimeDelta::days(1)),
                next_occurence.add(TimeDelta::days(1)),
            ),
            Periodicity::Week => (
                next_occurence.sub(TimeDelta::weeks(1)),
                next_occurence.add(TimeDelta::weeks(1)),
            ),
            Periodicity::Month => (
                next_occurence.checked_sub_months(Months::new(1)).unwrap(),
                next_occurence.checked_add_months(Months::new(1)).unwrap(),
            ),
            Periodicity::Year => (
                next_occurence.checked_sub_months(Months::new(12)).unwrap(),
                next_occurence.checked_add_months(Months::new(12)).unwrap(),
            ),
        };

        (prev_occurence, next_occurence, todo_occurence)
    }
    #[test]
    fn test_dates() {
        let (today, tommorow, _) = generate_prev_cur_next_occurence(Periodicity::Day, 1);
        println!("Day {today} {tommorow}");
        let (this_week, next_week, _) = generate_prev_cur_next_occurence(Periodicity::Week, 1);
        println!("Week {this_week} {next_week}");
        let (this_month, next_month, _) = generate_prev_cur_next_occurence(Periodicity::Month, 1);
        println!("Month {this_month} {next_month}");
        let (this_year, next_year, _) = generate_prev_cur_next_occurence(Periodicity::Year, 180);
        println!("Year {this_year} {next_year}");
    }
}
