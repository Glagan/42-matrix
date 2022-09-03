use crate::{linear_interpolation::Lerp, matrix::Matrix, norm::Norm};
use std::{
    fmt::{self, Debug},
    ops::{Add, Index, IndexMut, Mul, Sub},
    slice::Iter,
};

#[derive(Debug)]
pub struct Vector {
    elements: Vec<f64>,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self::new(0)
    }
}

// * Clone

impl Clone for Vector {
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }

    fn clone(&self) -> Self {
        Self {
            elements: self.elements.clone(),
        }
    }
}

// * Index access

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.elements[i]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.elements[i]
    }
}

// * Operations

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        if self.size() == rhs.size() {
            let mut vector = Vector::new(self.size());
            for index in 0..self.size() {
                vector[index] = self[index] + rhs[index];
            }
            vector
        } else {
            Vector::new(0)
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.size() == rhs.size() {
            let mut vector = Vector::new(self.size());
            for index in 0..self.size() {
                vector[index] = self[index] - rhs[index];
            }
            vector
        } else {
            Vector::new(0)
        }
    }
}

impl Mul for Vector {
    type Output = Vector;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.size() == rhs.size() {
            let mut vector = Vector::new(self.size());
            for index in 0..self.size() {
                vector[index] = self[index] * rhs[index];
            }
            vector
        } else {
            Vector::new(0)
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut vector = Vector::new(self.size());
        for index in 0..self.size() {
            vector[index] = self[index] * rhs;
        }
        vector
    }
}

// *> From

impl From<Vec<f64>> for Vector {
    fn from(vec: Vec<f64>) -> Self {
        Vector { elements: vec }
    }
}

impl<const N: usize> From<[f64; N]> for Vector {
    fn from(slice: [f64; N]) -> Self {
        Vector {
            elements: slice.to_vec(),
        }
    }
}

// *< From

// * Lerp

impl Lerp for Vector {
    fn lerp(a: &Vector, b: &Vector, t: f64) -> Vector {
        // if !(0. ..=1.).contains(&t) {
        //     return Err(format!(
        //         "Invalid t value of {}, should be between 0. and 1.",
        //         t
        //     ));
        // }

        if a.size() != b.size() {
            return Vector::new(0);
        }

        let mut result = Vector::new(a.size());
        for index in 0..a.size() {
            result[index] = a[index] * (1. - t) + b[index] * t;
        }
        result
    }
}

// * Vector

impl Vector {
    pub fn new(size: usize) -> Vector {
        Vector {
            elements: vec![f64::default(); size],
        }
    }

    // * Utility functions

    // Size of the vector -- number of columns of the vector
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    // Shape of the vector
    // Vector are assumed to be a single line with multiple columns
    #[allow(dead_code)]
    pub fn shape(&self) -> [usize; 2] {
        let size = self.size();
        if size > 0 {
            return [1, self.size()];
        }
        [0, 0]
    }

    // Transform the vector to a Matrix, keeping it's current size
    #[allow(dead_code)]
    pub fn reshape(&self) -> Matrix {
        Matrix::from(self.elements.clone())
    }

    #[allow(dead_code)]
    pub fn all(&self) -> &Vec<f64> {
        &self.elements
    }

    // Fill the vector with a given value
    #[allow(dead_code)]
    pub fn fill(&mut self, value: f64) {
        for element in self.elements.iter_mut() {
            *element = value
        }
    }

    // Create an iterator in the direction of the row of the vector
    #[allow(dead_code)]
    pub fn iter_rows(&self) -> Iter<'_, f64> {
        self.elements.iter()
    }

    // Create an iterator in the direction of the columns of the vector
    #[allow(dead_code)]
    pub fn iter_cols(&self) -> Iter<'_, f64> {
        self.elements.iter()
    }

    // Apply a function on each of the elements of the vector and return a new vector with the function applied
    #[allow(dead_code)]
    pub fn map(&self, callback: fn(index: usize, value: f64) -> f64) -> Result<Vector, String> {
        let size = self.size();
        let mut new_vector = Vector::new(size);
        for column in 0..size {
            new_vector[column] = callback(column, self[column]);
        }

        Ok(new_vector)
    }

    // Apply a function on each of the elements of two vectors and return a new vector with the function applied
    #[allow(dead_code)]
    pub fn map_tuple<'a>(
        a: &'a Vector,
        b: &'a Vector,
        callback: fn(index: usize, a: f64, b: f64) -> f64,
    ) -> Result<Vector, String> {
        let a_size = a.size();
        if a_size != b.size() {
            return Err(format!("Invalid sizes {:?} and {:?}", a_size, b.size()));
        }

        let mut new_vector = Vector::new(a_size);
        for column in 0..a_size {
            new_vector[column] = callback(column, a[column], b[column]);
        }

        Ok(new_vector)
    }

    // * Subject functions

    #[allow(dead_code)]
    pub fn add(&mut self, b: &Vector) -> Result<(), String> {
        let size = self.size();
        if size != b.size() {
            return Err(format!("Invalid sizes {:?} and {:?}", size, b.size()));
        }

        for column in 0..size {
            self[column] += b[column]
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn sub(&mut self, b: &Vector) -> Result<(), String> {
        let size = self.size();
        if size != b.size() {
            return Err(format!("Invalid sizes {:?} and {:?}", size, b.size()));
        }

        for column in 0..size {
            self[column] -= b[column]
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn scl(&mut self, value: f64) {
        let size = self.size();
        for column in 0..size {
            self[column] *= value
        }
    }

    #[allow(dead_code)]
    pub fn dot(&self, b: &Vector) -> f64 {
        let size = self.size();
        if size != b.size() {
            return f64::default();
        }

        let mut result = f64::default();
        for index in 0..size {
            result += self[index] * b[index];
        }
        result
    }

    #[allow(dead_code)]
    pub fn norm_1(&self) -> f64 {
        let mut result = 0.;
        for index in 0..self.size() {
            result += self[index].abs();
        }
        result
    }

    #[allow(dead_code)]
    pub fn norm(&self) -> f64 {
        let mut result = 0.;
        for index in 0..self.size() {
            result += self[index].pow(2.);
        }
        result.pow(0.5)
    }

    #[allow(dead_code)]
    pub fn norm_inf(&self) -> f64 {
        let mut result = 0.;
        if self.size() > 0 {
            result = self[0].abs()
        }
        for index in 1..self.size() {
            result = result.max(self[index].abs());
        }
        result
    }
}
