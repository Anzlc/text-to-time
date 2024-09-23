pub mod csv_parser;
pub mod prepare;
pub use prepare::*;
pub use csv_parser::read_and_parse_time;

#[derive(Clone)]
pub struct TimeData {
    natural_time: String,
    formatted_time: String,
}

impl TimeData {
    fn get_parsed_time(&self) -> [f64; 84] {
        let mut split = self.formatted_time.split(":");
        if split.clone().count() != 2 {
            panic!("Invalid time format: hh:mm - {}", self.formatted_time);
        }
        let mut res = [0.0; 84];

        let hours = split.next().unwrap().parse::<usize>().unwrap();
        let minutes = split.next().unwrap().parse::<usize>().unwrap();

        assert!(hours <= 24);
        assert!(minutes < 60);

        res[hours] = 1.0;
        res[minutes + 24] = 1.0;

        res
    }
}
