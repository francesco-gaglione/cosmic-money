use chrono::{Duration, NaiveDate};

pub fn get_month_date_range(year: i32, month: u32) -> (NaiveDate, NaiveDate) {
    let month_start =
        NaiveDate::from_ymd_opt(year, month, 1).expect("Data non valida per l'inizio del mese");

    let (next_year, next_month) = if month == 12 {
        (year + 1, 1) // Passa al nuovo anno
    } else {
        (year, month + 1)
    };

    let next_month_start = NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .expect("Data non valida per il mese successivo");

    let month_end = next_month_start - Duration::days(1);

    (month_start, month_end)
}
