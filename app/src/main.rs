use network::{ self, Network, SIGMOID };

fn main() {
    println!("Hello, world!");
    let mut network = Network::new(&[2, 3, 1], SIGMOID, 0.5);

    let inputs = vec![vec![0.0, 0.0], vec![0.0, 1.0], vec![1.0, 0.0], vec![1.0, 1.0]];
    let outputs = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];
    network.train(inputs, outputs, 60000);

    println!("Result for 0: {:?}", network.feed_forward(&[0.0, 0.0]));
    println!("Result for 1: {:?}", network.feed_forward(&[1.0, 1.0]));
    println!("Result for .5: {:?}", network.feed_forward(&[1.0, 0.0]));
    println!("Result for .5: {:?}", network.feed_forward(&[0.0, 1.0]));
}
