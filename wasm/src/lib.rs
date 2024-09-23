use std::{ cell::RefCell };

use wasm_bindgen::prelude::*;
use data_loader::TrainingData;
use network::{ self, Network, SIGMOID };

thread_local! {
    static NETWORK: RefCell<Option<Network<'static>>> = RefCell::new(None);
    static DATA: RefCell<Option<TrainingData>> = RefCell::new(None);
}

#[wasm_bindgen]
pub fn generate_response(input: String) -> String {
    setup_network();

    let input_data = DATA.with_borrow(|data| { data.as_ref().unwrap().to_input(&input) });
    let result = NETWORK.with_borrow_mut(|network| {
        network.as_mut().expect("Soksd ").feed_forward(&input_data)
    });

    TrainingData::from_output(&result)
}

fn setup_network() {
    let set_n = NETWORK.with_borrow(|x| {
        if x.is_none() {
            return true;
        }
        false
    });
    if set_n {
        let mut network: Network<'_> = Network::new(&[64, 256, 256, 84], SIGMOID, 0.125);
        network.load_from_bytes(include_bytes!("..\\..\\network.data"));
        NETWORK.set(Some(network));
    }
    let set_d = DATA.with_borrow(|x| {
        if x.is_none() {
            return true;
        }
        false
    });

    if set_d {
        let mut data = TrainingData::empty();
        data.load_map_bytes(include_bytes!("..\\..\\map.data"));
        DATA.set(Some(data));
    }
}

#[cfg(test)]
mod test {
    use crate::generate_response;

    #[test]
    fn reponse() {
        generate_response("pol osmih".to_string());
    }
}
