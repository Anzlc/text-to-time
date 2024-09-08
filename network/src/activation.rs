use std::f64::consts::E;

pub struct Activation<'a> {
    pub activation: &'a dyn Fn(f64) -> f64,
    pub derivative: &'a dyn Fn(f64) -> f64,
}

pub const SIGMOID: Activation = Activation {
    activation: &(|x| 1.0 / (1.0 + E.powf(-x))),
    derivative: &(|x| x * (1.0 - x)),
};
