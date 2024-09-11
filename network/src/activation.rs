use std::f64::consts::E;

pub struct Activation<'a> {
    pub activation: &'a dyn Fn(f64) -> f64,
    pub derivative: &'a dyn Fn(f64) -> f64,
}

pub const SIGMOID: Activation = Activation {
    activation: &(|x| 1.0 / (1.0 + E.powf(-x))),
    derivative: &(|x| x * (1.0 - x)),
};

pub const RELU: Activation = Activation {
    activation: &(|x| x.max(0.0)),
    derivative: &(|x| if x > 0.0 { 1.0 } else { 0.0 }),
};

pub const IDENTITY: Activation = Activation {
    activation: &(|x| x),
    derivative: &(|_| 1.0),
};
