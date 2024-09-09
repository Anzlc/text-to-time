use std::ops::{ Add, Mul, Sub };

use rand::{ self, Rng };

#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }
    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = rand::thread_rng();

        let mut res = Matrix::zeros(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                res.data[i][j] = rng.gen();
            }
        }
        res
    }
    pub fn dot_multiply(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to multiply with incorrect dimensions");
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }

        res
    }
    pub fn map(&self, function: &dyn Fn(f64) -> f64) -> Matrix {
        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = function(self.data[i][j]);
            }
        }

        res
    }
    pub fn transpose(&self) -> Matrix {
        let mut res = Matrix::zeros(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[j][i] = self.data[i][j];
            }
        }

        res
    }
    pub fn mul(&self, rhs: &Matrix) -> Matrix {
        if self.cols != rhs.rows {
            panic!("Attempted to multiply by matrix of incorrect dimensions");
        }
        let mut res = Matrix::zeros(self.rows, rhs.cols);

        for i in 0..self.rows {
            for j in 0..rhs.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * rhs.data[k][j];
                }
                res.data[i][j] = sum;
            }
        }

        res
    }
    pub fn add(&self, rhs: &Matrix) -> Matrix {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Attempted to add with incorrect dimensions");
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }

        res
    }
    pub fn sub(&self, rhs: &Matrix) -> Matrix {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Attempted to subtract with incorrect dimensions");
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }

        res
    }
}

// impl Add<&Matrix> for Matrix {
//     type Output = Matrix;

//     fn add(self, rhs: &Matrix) -> Self::Output {
//         if self.rows != rhs.rows || self.cols != rhs.cols {
//             panic!("Attempted to add with incorrect dimensions");
//         }

//         let mut res = Matrix::zeros(self.rows, self.cols);

//         for i in 0..self.rows {
//             for j in 0..self.cols {
//                 res.data[i][j] = self.data[i][j] + rhs.data[i][j];
//             }
//         }

//         res
//     }
// }

// impl Sub<&Matrix> for Matrix {
//     type Output = Matrix;

//     fn sub(self, rhs: &Matrix) -> Matrix {
//         if self.rows != rhs.rows || self.cols != rhs.cols {
//             panic!("Attempted to subtract with incorrect dimensions");
//         }

//         let mut res = Matrix::zeros(self.rows, self.cols);

//         for i in 0..self.rows {
//             for j in 0..self.cols {
//                 res.data[i][j] = self.data[i][j] - rhs.data[i][j];
//             }
//         }

//         res
//     }
// }

// impl Mul<&Matrix> for Matrix {
//     type Output = Matrix;

//     fn mul(self, rhs: &Matrix) -> Matrix {
//         if self.cols != rhs.rows {
//             panic!("Attempted to multiply by matrix of incorrect dimensions");
//         }
//         let mut res = Matrix::zeros(self.rows, rhs.cols);

//         for i in 0..self.rows {
//             for j in 0..rhs.cols {
//                 let mut sum = 0.0;
//                 for k in 0..self.cols {
//                     sum += self.data[i][k] * rhs.data[k][j];
//                 }
//                 res.data[i][j] = sum;
//             }
//         }

//         res
//     }
// }

impl From<Vec<Vec<f64>>> for Matrix {
    fn from(data: Vec<Vec<f64>>) -> Matrix {
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data,
        }
    }
}
