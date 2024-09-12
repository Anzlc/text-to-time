use std::fs::read_to_string;

use crate::TimeData;

pub fn read_and_parse_time(path: &str) -> Vec<TimeData> {
    let data = read_to_string(path).unwrap_or(String::from("natural,time"));

    let mut lines = data.lines();
    let mut entries = vec![];

    lines.next(); // Skip first line

    for line in lines {
        let mut parts = line.split(",");

        let natural_time = parts.next().unwrap();
        let formatted_time = parts.next().unwrap();

        entries.push(TimeData {
            natural_time: natural_time.to_lowercase().to_owned(),
            formatted_time: formatted_time.to_owned(),
        });
    }

    entries
}
