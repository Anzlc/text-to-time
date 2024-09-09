use crate::TimeData;

pub fn generate_training_data(data: &[TimeData]) -> (Vec<[f64; 64]>, Vec<[f64; 4]>) {
    let mut inputs: Vec<[f64; 64]> = Vec::new();
    let mut outputsVec: Vec<[f64; 4]> = Vec::new();

    for time in data {
        let mut natural_input: [f64; 64] = [0.0; 64];
        if time.natural_time.len() > 64 {
            panic!("Input \"{}\" exceeded 64 characters!", time.natural_time);
        }
        for c in time.natural_time.chars() {
        }
    }

    (vec![], vec![])
}
