use std::u16;

use network::{ IDENTITY, RELU };
use network::{ self, Network, SIGMOID };
use data_loader::TrainingData;
use data_loader::read_and_parse_time;
fn main() {
    println!("Hello, world!");
    let mut training_data = TrainingData::generate_training_data(
        &read_and_parse_time("./data.csv")
    );
    let mut network: Network<'_> = Network::new(&[64, 32, 4, 2], SIGMOID, 0.01);
    let mut prev_best = 0;
    loop {
        network.train(
            &training_data.get_training_input(),
            &training_data.get_training_output(),
            100
        );
        let mut correct = 0;
        for (i, inputs) in training_data.get_training_input().into_iter().enumerate() {
            if network.feed_forward(&inputs) == training_data.get_training_output()[i] {
                correct += 1;
            }
        }
        println!("Correctnes: {}", correct);
        if correct >= prev_best {
            prev_best = correct;
        } else {
            break;
        }
    }

    println!("Correctness {}/{}", prev_best, training_data.get_training_input().len());
    loop {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        let result = network.feed_forward(&training_data.to_input(&input));
        let mut res_iter = result.into_iter();
        println!(
            "{}: {}",
            input,
            TrainingData::from_output(&[res_iter.next().unwrap(), res_iter.next().unwrap()])
        );
    }
}
