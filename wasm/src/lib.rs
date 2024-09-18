use std::{ borrow::Borrow, cell::RefCell };

use wasm_bindgen::prelude::*;
use data_loader::TrainingData;
use network::{ self, Network, SIGMOID };

thread_local! {
    static NETWORK: RefCell<Option<Network<'static>>> = RefCell::new(None);
    static DATA: RefCell<Option<TrainingData>> = RefCell::new(None);
}

#[wasm_bindgen]
pub fn generate_response(input: &str) -> String {
    setup_network();
    // let result = network.feed_forward(
    //              &training_data.to_input(&input.trim()));

    // println!("\r{}: {}", input.trim(), TrainingData::from_output(&result));

    let input_data = DATA.with_borrow(|data| { data.as_ref().unwrap().to_input(input) });
    let result = NETWORK.with_borrow_mut(|network| {
        network.as_mut().unwrap().feed_forward(&input_data)
    });

    TrainingData::from_output(&result)
}

fn setup_network() {
    NETWORK.with_borrow_mut(|x| {
        if x.is_none() {
            let mut network: Network<'_> = Network::new(&[64, 256, 256, 84], SIGMOID, 0.125);
            network.load_from_bytes(include_bytes!("..\\network.data"));
            NETWORK.set(Some(network));
        }
    });
    DATA.with_borrow_mut(|x| {
        if x.is_none() {
            let mut data = TrainingData::empty();
            data.load_map_bytes(include_bytes!("..\\data.data"));
            DATA.set(Some(data));
        }
    });
}
