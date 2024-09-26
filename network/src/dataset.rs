pub trait DataSet {
    fn get_training_data(&self) -> (Vec<Vec<f64>>, Vec<Vec<f64>>);
    fn get_testing_data(&self) -> (Vec<Vec<f64>>, Vec<Vec<f64>>);
}
