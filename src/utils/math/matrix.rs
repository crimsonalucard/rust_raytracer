use std::ops::{Index, IndexMut, Mul};
use std::fmt::{Display, Formatter, Result};
use std::cmp::Eq;
use crate::utils::math::tuple::Tuple4;

pub struct Matrix {
    pub value: Vec<Vec<f64>>
}


fn zero(n: usize, m: usize) -> Matrix {
    let acc: Vec<Vec<f64>> = vec![vec![0.0; m]; n];
    Matrix {
        value: acc
    }
}

fn identity(n: usize, m: usize) -> Matrix {
    let mut acc: Vec<Vec<f64>> = vec![vec![0.0; m]; n];
    for i in 0..n {
        acc[i][i] = 1.0;
    }
    let acc = acc;
    Matrix {
        value: acc
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut acc = String::from("");
        for i in 0..self.value.len() {
            acc.push_str("| ");
            for val in &self.value[i] {
                acc.push_str(&format!("{}", val));
                acc.push(' ');
            }
            acc.push_str("|\n");
        }
        write!(f, "{}", acc)
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let mut acc = vec![vec![0.0; rhs.value[0].len()]; self.value.len()];
        for c_row in 0..acc.len() {
            for c_col in 0..acc[0].len() {
                for l in &self.value[c_row] {
                    for r in 0..rhs.value.len() {
                        acc[c_row][c_col] += l * rhs.value[r][c_col];
                    }
                }
            }
        }

        Matrix {
            value: acc
        }
    }
}

impl Mul<Tuple4> for Matrix {
    type Output = Tuple4;

    fn mul(self, rhs: Tuple4) -> Self::Output {
        let mut result = Tuple4 {
            value: [0.0; 4]
        };

        for (index, matrix_row) in self.value.iter().enumerate() {
            for (tuple_value, matrix_value) in rhs.value.iter().zip(matrix_row) {
                result.value[index] += tuple_value * matrix_value
            }
        }
        result
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        self * &rhs
    }
}

impl Mul<&Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        &self * rhs
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.value[index]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.value.len() == 0 && other.value.len() == 0 {
            return true;
        } else if self.value.len() == 0 || other.value.len() == 0 {
            return false;
        } else {
            let mut result = true;
            for i in 0..self.value.len() {
                for j in 0..self.value[0].len() {
                    result &= self[i][j] == other[i][j]
                }
            }
            result
        }
    }
}

impl Eq for Matrix {}

#[test]
fn test_matrix_equality() {
    let i = &identity(4, 4);
    print!("{}", i);
    assert!(i == i)
}

#[test]
fn test_multiplication() {
    let i = identity(4, 4);
    let j = identity(4, 4);
    let k = identity(4, 4);
    print!("{}", i * j)
    // assert!(i*j == k)
}

