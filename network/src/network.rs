use std::{ fs::File, io::{ self, BufReader, Read, Write }, vec };
use crate::{ activation::Activation, dataset::DataSet, matrix::Matrix };

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

    pub fn train_with_testing<T>(
        &mut self,
        data_set: &T,
        file: &str,
        test_function: &dyn Fn(&[f64], &[f64]) -> bool
    )
        where T: DataSet
    {
        let mut prev_best = 0;
        let (inputs_training, targets_training) = data_set.get_training_data();
        let (inputs_testing, targets_testing) = data_set.get_testing_data();

        loop {
            for _ in 0..10 {
                for j in 0..inputs_training.len() {
                    let outputs = self.feed_forward(&inputs_training[j]);

                    self.back_propagate(&outputs, &targets_training[j]);
                }
            }

            let mut correct = 0;
            for (i, inputs) in inputs_testing.iter().enumerate() {
                if test_function(&self.feed_forward(&inputs), &targets_testing[i]) {
                    correct += 1;
                }
            }

            if correct < prev_best {
                continue;
            } else if correct > prev_best {
                prev_best = correct;

                match self.save(file) {
                    Ok(_) =>
                        println!("\nSaved with a score of {}/{}", correct, targets_testing.len()),
                    Err(_) => println!("Failed to save model!"),
                }
            }

            print!("Current best: {}, current: {}\r", prev_best, correct);
            let _ = std::io::stdout().flush();
        }
    }

    pub fn save(&self, file: &str) -> Result<(), io::Error> {
        let mut file = File::create(file)?;
        let mut buffer: Vec<u8> = Vec::new();
        for i in 0..self.layers.len() - 1 {
            let matrix = &self.weights[i];
            for rows in 0..matrix.rows {
                for cols in 0..matrix.cols {
                    let _ = buffer.write(&matrix.data[rows][cols].to_be_bytes());
                }
            }
        }
        for i in 0..self.layers.len() - 1 {
            let matrix = &self.biases[i];
            for rows in 0..matrix.rows {
                for cols in 0..matrix.cols {
                    let _ = buffer.write(&matrix.data[rows][cols].to_be_bytes());
                }
            }
        }

        file.write(&buffer)?;
        Ok(())
    }
    pub fn load(&mut self, path: &str) -> Result<(), io::Error> {
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut data = Vec::new();
        buf_reader.read_to_end(&mut data)?;
        self.load_from_bytes(&data);
        Ok(())
    }

    pub fn load_from_bytes(&mut self, data: &[u8]) {
        let mut data = data.chunks(8).map(|x: &[u8]| f64::from_be_bytes(x.try_into().unwrap()));

        for i in 0..self.layers.len() - 1 {
            let matrix = &mut self.weights[i];
            for rows in 0..matrix.rows {
                for cols in 0..matrix.cols {
                    //matrix.data[rows][cols] = f64::from_be_bytes(data[rows * cols + cols]);
                    //matrix.data[rows][cols] = data[rows * matrix.rows + cols];
                    matrix.data[rows][cols] = data.next().unwrap();
                }
            }
        }

        for i in 0..self.layers.len() - 1 {
            let matrix = &mut self.biases[i];
            for rows in 0..matrix.rows {
                for cols in 0..matrix.cols {
                    // matrix.data[rows][cols] = data[rows * matrix.rows + cols + offset];
                    matrix.data[rows][cols] = data.next().unwrap();
                }
            }
        }
    }
}
