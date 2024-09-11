pub mod csv_parser;
pub mod prepare;
pub use prepare::*;
pub use csv_parser::read_and_parse_time;

pub struct TimeData {
    natural_time: String,
    formatted_time: String,
}

impl TimeData {
    fn get_parsed_time(&self) -> [f64; 2] {
        let mut split = self.formatted_time.split(":");
        if split.clone().count() != 2 {
            panic!("Invalid time format: hh:mm - {}", self.formatted_time);
        }
        [
            split.next().unwrap().parse::<f64>().unwrap() / 24.0,
            split.next().unwrap().parse::<f64>().unwrap() / 60.0,
        ]
    }
}
