#[cfg(test)]
mod tests {
    use crate::application_enums::Periodicity;
    use chrono::{Datelike, Months, NaiveDate, TimeDelta, Utc};
    use cron::Schedule;
    use std::ops::Add;
    use std::str::FromStr;

    fn from_cron(cron: String) -> NaiveDate {
        Schedule::from_str(cron.as_str())
            .unwrap()
            .upcoming(Utc)
            .next()
            .unwrap()
            .date_naive()
    }

    fn get_next_occurence_with_offset(
        periodicity: &Periodicity,
        date: &NaiveDate,
        offset: i32,
    ) -> NaiveDate {
        match periodicity {
            Periodicity::Day => date.add(TimeDelta::days(offset as i64)),
            Periodicity::Week => date.add(TimeDelta::weeks(offset as i64)),
            Periodicity::Month => {
                if offset >= 0 {
                    date.checked_add_months(Months::new(offset as u32)).unwrap()
                } else {
                    date.checked_sub_months(Months::new((-offset) as u32))
                        .unwrap()
                }
            }
            Periodicity::Year => {
                if offset >= 0 {
                    date.checked_add_months(Months::new(12 * offset as u32))
                        .unwrap()
                } else {
                    date.checked_sub_months(Months::new(12 * (-offset) as u32))
                        .unwrap()
                }
            }
        }
    }

    fn generate_occurences(
        periodicity: Periodicity,
        periodicity_unit: u32,
        future_count: u32,
    ) -> Vec<NaiveDate> {
        let next_occurence = match periodicity {
            Periodicity::Day => from_cron("0 0 0 * * * *".to_string()), // periodicity_unit is irrelevant
            Periodicity::Week => from_cron(format!("0 0 0 * * {periodicity_unit} *")), // periodicity_unit is day_of_week 0 = sunday, 6=saturday
            Periodicity::Month => from_cron(format!("0 0 0 {periodicity_unit} * * *")), // periodicity_unit is day_of_month 0 = sunday, 6=saturday
            Periodicity::Year => NaiveDate::from_ymd_opt(Utc::now().year(), 1, 1)
                .unwrap()
                .add(TimeDelta::days(periodicity_unit as i64)), // periodicity_unit is day_of_year
        };

        let mut occurences = Vec::<NaiveDate>::with_capacity(future_count as usize);
        for x in -1..=(future_count as i32 - 2) {
            occurences.push(get_next_occurence_with_offset(
                &periodicity,
                &next_occurence,
                x,
            ));
        }

        occurences
    }
    #[test]
    fn test_dates() {
        println!("Day {:?}", generate_occurences(Periodicity::Day, 1, 3));
        println!("Week {:?}", generate_occurences(Periodicity::Week, 1, 3));
        println!("Month {:?}", generate_occurences(Periodicity::Month, 1, 3));
        println!("Month {:?}", generate_occurences(Periodicity::Year, 1, 3));
    }
}
