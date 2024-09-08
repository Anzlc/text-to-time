use std::vec;

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
            current = self.weights[i]
                .mul(&current)
                .add(&self.biases[i])
                .map(&self.activation.activation);
            self.data.push(current.clone());
        }

        current.data[0].to_owned()
    }

    pub fn back_propagate(&mut self, outputs: Vec<f64>, targets: Vec<f64>) {
        if targets.len() != *self.layers.last().unwrap() {
            panic!("Invalid number of targets");
        }

        let mut parsed = Matrix::from(vec![outputs]);
        let mut errors = Matrix::from(vec![targets]);
        let mut gradients = parsed.map(self.activation.derivative);

        for i in (0..self.layers.len() - 1).rev() {
            gradients = gradients.dot_multiply(&errors).map(&(|x| x * self.learning_rate));
            self.weights[i] = self.weights[i].add(&gradients.mul(&self.data[i].transpose()));
            self.biases[i] = self.biases[i].add(&gradients);

            errors = self.weights[i].transpose().mul(&errors);
            gradients = self.data[i].map(self.activation.derivative);
        }
    }

    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u16) {
        for i in 1..=epochs {
            print!("Epoch {}/{}\r", i, epochs);

            for j in 0..inputs.len() {
                let outputs = self.feed_forward(&inputs[j]);
                self.back_propagate(outputs, targets[j].clone());
            }
        }
    }
}
