use std::collections::HashMap;

use crate::TimeData;

pub fn generate_training_data(data: &[TimeData]) -> (Vec<[f64; 64]>, Vec<[f64; 2]>) {
    let mut inputs: Vec<[f64; 64]> = Vec::new();
    let mut outputs: Vec<[f64; 2]> = Vec::new();

    let mut character_map: HashMap<char, f64> = HashMap::new();

    for (i, time) in data.into_iter().enumerate() {
        let mut natural_input: [f64; 64] = [0.0; 64];
        let mut formatted_output: [f64; 2] = [0.0; 2];
        if time.natural_time.len() > 64 {
            panic!("Input \"{}\" exceeded 64 characters!", time.natural_time);
        }
        for (j, c) in time.natural_time.chars().enumerate() {
            if !character_map.contains_key(&c) {
                character_map.insert(c, character_map.len() as f64);
            }
            natural_input[j] = *character_map.get(&c).unwrap();
        }
        inputs.push(natural_input);
        outputs.push(time.get_parsed_time());
    }

    (inputs, outputs)
}
