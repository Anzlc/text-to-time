use network::{ IDENTITY, RELU };
use network::{ self, Network, SIGMOID };
use data_loader::TrainingData;
use data_loader::read_and_parse_time;
fn main() {
    println!("Hello, world!");
    let mut training_data = TrainingData::generate_training_data(
        &read_and_parse_time("./data.csv")
    );
    let mut network = Network::new(&[64, 128, 64, 32, 16, 2], RELU, 0.01);

    network.train(&training_data.get_training_input(), &training_data.get_training_output(), 1000);

    let tests = vec!["deset čez pet", "pol desetih", "polnoč"];

    for test in tests {
        println!("{:?}", &training_data.to_input(test));
        let mut result = network.feed_forward(&training_data.to_input(test));
        println!("{:?}", result);
        let mut res_iter = result.into_iter();
        println!(
            "{}: {}",
            test,
            TrainingData::from_output(&[res_iter.next().unwrap(), res_iter.next().unwrap()])
        );
    }
}
