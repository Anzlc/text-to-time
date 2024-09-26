use network::{ self, Network, SIGMOID };
use data_loader::TrainingData;
use data_loader::read_and_parse_time;
fn main() {
    println!("Hello, world!");
    let data = read_and_parse_time("./data.csv");
    let training_data = TrainingData::generate_training_data(&data);
    let mut network: Network<'_> = Network::new(&[64, 256, 256, 84], SIGMOID, 0.1);
    training_data.save_map("map.data");

    network.train_with_testing(
        &training_data,
        "network.data",
        &(|x, y| { TrainingData::from_output(x) == TrainingData::from_output(y) })
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
