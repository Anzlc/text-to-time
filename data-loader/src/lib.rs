mod csv_parser;
mod prepare;

struct TimeData {
    natural_time: String,
    formatted_time: String,
}

impl TimeData {
    fn get_parsed_time(&self) -> [f64; 2] {
        if self.formatted_time.len() != 5 {
            panic!("Invalid time format: hh:mm");
        }
        let mut split = self.formatted_time.split(":");
        [split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap()]
    }
}
