use std::{ io::Write, vec };
use data_loader::TrainingData;
use crate::{ activation::{ self, Activation, SIGMOID }, matrix::Matrix };

pub struct Network<'a> {
    layers: &'a [usize],
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    activation: Activation<'a>,
    learning_rate: f64,
}

impl Network<'_> {
    pub fn new<'a>(
        layers: &'a [usize],
        activation: Activation<'a>,
        learning_rate: f64
    ) -> Network<'a> {
        let mut weights = Vec::with_capacity(layers.len() - 1);
        let mut biases = Vec::with_capacity(layers.len() - 1);

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::random(layers[i + 1], layers[i]));
            biases.push(Matrix::random(layers[i + 1], 1));
        }

        Network {
            layers,
            weights,
            biases,
            data: vec![],
            activation,
            learning_rate,
        }
    }

    pub fn feed_forward(&mut self, inputs: &[f64]) -> Vec<f64> {
        if self.layers[0] != inputs.len() {
            panic!("Invalid number of inputs");
        }

        let mut current = Matrix::from(vec![inputs.to_owned()]).transpose();
        self.data = vec![current.clone()];
        for i in 0..self.layers.len() - 1 {
            let tmp = current.clone();
            current = self.weights[i]
                .mul(&current)
                .add(&self.biases[i])
                .map(&self.activation.activation);
            self.data.push(current.clone());
        }

        current.transpose().data[0].to_owned()
    }

    pub fn back_propagate(&mut self, outputs: &[f64], targets: &[f64]) {
        if targets.len() != *self.layers.last().unwrap() {
            panic!("Invalid number of targets");
        }

        let mut parsed = Matrix::from(vec![outputs.to_owned()]).transpose();
        let mut errors = Matrix::from(vec![targets.to_owned()]).transpose().sub(&parsed);
        let mut gradients = parsed.map(self.activation.derivative);

        for i in (0..self.layers.len() - 1).rev() {
            gradients = gradients.dot_multiply(&errors).map(&(|x| x * self.learning_rate));
            // gradients = gradients.map(
            //     &(|x| if x > 1.0 { 1.0 } else if x < -1.0 { -1.0 } else { x })
            // );
            self.weights[i] = self.weights[i].add(&gradients.mul(&self.data[i].transpose()));
            self.biases[i] = self.biases[i].add(&gradients);

            errors = self.weights[i].transpose().mul(&errors);
            gradients = self.data[i].map(self.activation.derivative);
        }
    }

    pub fn train(&mut self, inputs: &Vec<Vec<f64>>, targets: &Vec<Vec<f64>>, epochs: u16) {
        println!("Started training");
        if inputs.len() != targets.len() {
            panic!("Length of inputs and targets does not match");
        }

        for i in 1..=epochs {
            print!("Epoch {}/{}\r", i, epochs);
            std::io::stdout().flush();
            for j in 0..inputs.len() {
                let outputs = self.feed_forward(&inputs[j]);

                self.back_propagate(&outputs, &targets[j]);
            }
        }
        println!("Finished training");
    }

    pub fn train_with_testing(
        &mut self,
        inputs_training: &Vec<Vec<f64>>,
        targets_training: &Vec<Vec<f64>>,
        inputs_testing: &Vec<Vec<f64>>,
        targets_testing: &Vec<Vec<f64>>
    ) {
        let mut prev_best = 0;
        loop {
            for _ in 0..200 {
                for j in 0..inputs_training.len() {
                    let outputs = self.feed_forward(&inputs_training[j]);

                    self.back_propagate(&outputs, &targets_training[j]);
                }
            }

            let mut correct = 0;
            for (i, inputs) in inputs_testing.into_iter().enumerate() {
                if
                    TrainingData::from_output(&self.feed_forward(&inputs)) ==
                    TrainingData::from_output(&targets_testing[i])
                {
                    correct += 1;
                }
                //println!("Got right: {}", TrainingData::from_output(&self.feed_forward(&inputs)));
            }
            if correct < prev_best {
                break;
            }
            prev_best = correct;
            print!("Current best: {}\r", prev_best);
            std::io::stdout().flush();
        }
        println!("");
    }
}
