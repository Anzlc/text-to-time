use network::{ self, Network, SIGMOID };
use data_loader::TrainingData;
use data_loader::read_and_parse_time;
fn main() {
    println!("Hello, world!");
    let data = read_and_parse_time("./data.csv");
    let training_data = TrainingData::generate_training_data(&data);
    let mut network: Network<'_> = Network::new(&[64, 256, 256, 84], SIGMOID, 0.1);
    training_data.save_map("map.data");
    const TRAINING_PERCENTAGE: f64 = 0.9;
    let testing_count = (
        (training_data.get_training_input().len() as f64) * TRAINING_PERCENTAGE
    ).floor() as usize;

    let training_input = &training_data.get_training_input()[..testing_count];
    let testing_input = &training_data.get_training_input()[testing_count..];
    let training_output = &training_data.get_training_output()[..testing_count];
    let testing_output = &training_data.get_training_output()[testing_count..];

    network.train_with_testing_and_save(
        &training_input.to_vec(),
        &training_output.to_vec(),
        &testing_input.to_vec(),
        &testing_output.to_vec(),
        "network.data"
    );

    //network.save("network.data");
    //network.load("network.data");

    loop {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        let result = network.feed_forward(&training_data.to_input(&input.trim()));

        println!("\r{}: {}", input.trim(), TrainingData::from_output(&result));
    }
}
