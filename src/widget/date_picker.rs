use chrono::{Datelike, Local, NaiveDateTime, TimeZone, Timelike, Utc};
use cosmic::{widget, Element};

const DAYS: [&str; 31] = [
    "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15", "16",
    "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31",
];

const MONTHS: [&str; 12] = [
    "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12",
];

const HOURS: [&str; 24] = [
    "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15",
    "16", "17", "18", "19", "20", "21", "22", "23",
];

const MINUTES: [&str; 60] = [
    "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15",
    "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31",
    "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47",
    "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
];

pub fn date_picker<'a, M>(
    timestamp: i64,
    on_input: impl Fn(i64) -> M + 'a + Clone,
) -> Element<'a, M>
where
    M: 'static + Clone,
{
    let validate_datetime_input =
        move |day: usize, month: usize, year: usize, hour: usize, minute: usize| -> Option<i64> {
            if day < 1 || month < 1 || year < 1970 || hour > 23 || minute > 59 {
                return None;
            }
            let combined_input = format!(
                "{:02}/{:02}/{:04} {:02}:{:02}",
                day, month, year, hour, minute
            );
            match NaiveDateTime::parse_from_str(&combined_input, "%d/%m/%Y %H:%M") {
                Ok(parsed_datetime) => {
                    let local_datetime = Local.from_local_datetime(&parsed_datetime).unwrap();
                    let utc_timestamp = local_datetime.with_timezone(&Utc).timestamp();
                    Some(utc_timestamp)
                }
                Err(_) => None,
            }
        };

    let datetime = match Local.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt.naive_local(),
        _ => Local.ymd(1970, 1, 1).and_hms(0, 0, 0).naive_local(),
    };

    let day = datetime.day();
    let month = datetime.month();
    let year = datetime.year();
    let hour = datetime.hour();
    let minute = datetime.minute();

    let mut element = widget::row::<M>().spacing(10);

    element = element.push(
        widget::row::<M>()
            .spacing(5)
            .push(widget::text("gg:"))
            .push(widget::dropdown(&DAYS, Some((day - 1) as usize), {
                let on_input = on_input.clone();
                move |selected_day| {
                    if let Some(valid_timestamp) = validate_datetime_input(
                        selected_day + 1,
                        month as usize,
                        year as usize,
                        hour as usize,
                        minute as usize,
                    ) {
                        on_input(valid_timestamp)
                    } else {
                        on_input(timestamp)
                    }
                }
            })),
    );

    element = element.push(
        widget::row::<M>()
            .spacing(5)
            .push(widget::text("MM:"))
            .push(widget::dropdown(&MONTHS, Some((month - 1) as usize), {
                let on_input = on_input.clone(); // Cloniamo on_input
                move |selected_month| {
                    if let Some(valid_timestamp) = validate_datetime_input(
                        day as usize,
                        selected_month + 1,
                        year as usize,
                        hour as usize,
                        minute as usize,
                    ) {
                        on_input(valid_timestamp)
                    } else {
                        on_input(timestamp)
                    }
                }
            })),
    );

    element = element.push(
        widget::row::<M>()
            .spacing(5)
            .push(widget::text("yyyy:"))
            .push(widget::dropdown(
                &[
                    "1970", "1971", "1972", "1973", "1974", "1975", "1976", "1977", "1978", "1979",
                    "1980", "1981", "1982", "1983", "1984", "1985", "1986", "1987", "1988", "1989",
                    "1990", "1991", "1992", "1993", "1994", "1995", "1996", "1997", "1998", "1999",
                    "2000", "2001", "2002", "2003", "2004", "2005", "2006", "2007", "2008", "2009",
                    "2010", "2011", "2012", "2013", "2014", "2015", "2016", "2017", "2018", "2019",
                    "2020", "2021", "2022", "2023", "2024",
                ],
                Some((year - 1970) as usize),
                {
                    let on_input = on_input.clone();
                    move |selected_year| {
                        if let Some(valid_timestamp) = validate_datetime_input(
                            day as usize,
                            month as usize,
                            selected_year + 1970,
                            hour as usize,
                            minute as usize,
                        ) {
                            on_input(valid_timestamp)
                        } else {
                            on_input(timestamp)
                        }
                    }
                },
            )),
    );

    element = element.push(
        widget::row::<M>()
            .spacing(5)
            .push(widget::text("HH:"))
            .push(widget::dropdown(&HOURS, Some(hour as usize), {
                let on_input = on_input.clone();
                move |selected_hour| {
                    if let Some(valid_timestamp) = validate_datetime_input(
                        day as usize,
                        month as usize,
                        year as usize,
                        selected_hour,
                        minute as usize,
                    ) {
                        on_input(valid_timestamp)
                    } else {
                        on_input(timestamp)
                    }
                }
            })),
    );

    element = element.push(
        widget::row::<M>()
            .spacing(5)
            .push(widget::text("mm:"))
            .push(widget::dropdown(&MINUTES, Some(minute as usize), {
                let on_input = on_input.clone();
                move |selected_minute| {
                    if let Some(valid_timestamp) = validate_datetime_input(
                        day as usize,
                        month as usize,
                        year as usize,
                        hour as usize,
                        selected_minute,
                    ) {
                        on_input(valid_timestamp)
                    } else {
                        on_input(timestamp)
                    }
                }
            })),
    );

    element.into()
}
