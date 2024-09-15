use std::collections::{ HashMap, HashSet };

use crate::TimeData;

pub struct TrainingData {
    character_map: HashMap<char, f64>,
    inputs: Vec<[f64; 64]>,
    outputs: Vec<[f64; 84]>,
}
impl TrainingData {
    pub fn generate_training_data(data: &[TimeData]) -> TrainingData {
        let mut inputs: Vec<[f64; 64]> = Vec::new();
        let mut outputs: Vec<[f64; 84]> = Vec::new();

        let mut character_map: HashMap<char, f64> = HashMap::new();

        for (i, time) in data.into_iter().enumerate() {
            let mut natural_input: [f64; 64] = [0.0; 64];
            let mut formatted_output: [f64; 84] = [0.0; 84];
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
        for input in inputs.iter_mut() {
            for i in 0..64 {
                if character_map.len() == 0 {
                    panic!("Da faq");
                }
                input[i] /= character_map.len() as f64;
            }
        }
        TrainingData {
            character_map,
            inputs,
            outputs,
        }
    }

    pub fn from_output(output: &[f64]) -> String {
        assert!(output.len() == 84);
        let hours = &output[0..24];
        let minutes = &output[24..84];

        let hours2 = hours
            .into_iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .unwrap();
        let minutes2 = minutes
            .into_iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .unwrap();

        format!("{:0>2}:{:0>2}", hours2, minutes2)
    }

    pub fn to_input(&self, data: &str) -> [f64; 64] {
        let mut natural_input: [f64; 64] = [0.0; 64];

        if data.len() > 64 {
            panic!("Input \"{}\" exceeded 64 characters!", data);
        }
        for (j, c) in data.chars().enumerate() {
            natural_input[j] =
                *self.character_map.get(&c).unwrap_or(&0.0) / (self.character_map.len() as f64);
        }

        natural_input
    }

    pub fn get_training_input(&self) -> Vec<Vec<f64>> {
        self.inputs
            .clone()
            .into_iter()
            .map(|x| x.to_vec())
            .collect()
    }

    pub fn get_training_output(&self) -> Vec<Vec<f64>> {
        self.outputs
            .clone()
            .into_iter()
            .map(|x| x.to_vec())
            .collect()
    }
}
