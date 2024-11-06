use chrono::{Duration, NaiveDate};

pub fn get_month_date_range(year: i32, month: u32) -> (NaiveDate, NaiveDate) {
    let month_start =
        NaiveDate::from_ymd_opt(year, month, 1).expect("Data non valida per l'inizio del mese");

    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt((month + 1) as i32, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }
    .expect("Data non valida per il primo giorno del mese successivo");

    let month_end = next_month - Duration::days(1);

    (month_start, month_end)
}
