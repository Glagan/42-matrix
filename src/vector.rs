use crate::{linear_interpolation::Lerp, matrix::Matrix, norm::Norm};
use std::{
    fmt::{self, Debug},
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
    slice::Iter,
};

#[derive(Debug)]
pub struct Vector<
    K: Default
        + Clone
        + Copy
        + Debug
        + AddAssign
        + SubAssign
        + MulAssign
        + Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + Mul<f64, Output = K>
        + Norm,
> {
    elements: Vec<K>,
}

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > fmt::Display for Vector<K>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Default for Vector<K>
{
    fn default() -> Self {
        Self::new(0)
    }
}

// * Clone

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Clone for Vector<K>
{
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

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Index<usize> for Vector<K>
{
    type Output = K;

    fn index(&self, i: usize) -> &K {
        &self.elements[i]
    }
}

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > IndexMut<usize> for Vector<K>
{
    fn index_mut(&mut self, i: usize) -> &mut K {
        &mut self.elements[i]
    }
}

// * Operations

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Add for Vector<K>
{
    type Output = Vector<K>;

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

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Sub for Vector<K>
{
    type Output = Vector<K>;

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

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Mul for Vector<K>
{
    type Output = Vector<K>;

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

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Mul<f64> for Vector<K>
{
    type Output = Vector<K>;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut vector = Vector::new(self.size());
        for index in 0..self.size() {
            vector[index] = self[index] * rhs;
        }
        vector
    }
}

// *> From

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > From<Vec<K>> for Vector<K>
{
    fn from(vec: Vec<K>) -> Self {
        Vector { elements: vec }
    }
}

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Mul<f64, Output = K>
            + Norm,
        const N: usize,
    > From<[K; N]> for Vector<K>
{
    fn from(slice: [K; N]) -> Self {
        Vector {
            elements: slice.to_vec(),
        }
    }
}

// *< From

// * Lerp

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Lerp for Vector<K>
{
    fn lerp(a: &Vector<K>, b: &Vector<K>, t: f64) -> Vector<K> {
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

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Vector<K>
{
    pub fn new(size: usize) -> Vector<K> {
        Vector {
            elements: vec![K::default(); size],
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
    pub fn reshape(&self) -> Matrix<K> {
        Matrix::from(self.elements.clone())
    }

    #[allow(dead_code)]
    pub fn all(&self) -> &Vec<K> {
        &self.elements
    }

    // Fill the vector with a given value
    #[allow(dead_code)]
    pub fn fill(&mut self, value: K) {
        for element in self.elements.iter_mut() {
            *element = value
        }
    }

    // Create an iterator in the direction of the row of the vector
    #[allow(dead_code)]
    pub fn iter_rows(&self) -> Iter<'_, K> {
        self.elements.iter()
    }

    // Create an iterator in the direction of the columns of the vector
    #[allow(dead_code)]
    pub fn iter_cols(&self) -> Iter<'_, K> {
        self.elements.iter()
    }

    // Apply a function on each of the elements of the vector and return a new vector with the function applied
    #[allow(dead_code)]
    pub fn map(&self, callback: fn(index: usize, value: K) -> K) -> Result<Vector<K>, String> {
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
        a: &'a Vector<K>,
        b: &'a Vector<K>,
        callback: fn(index: usize, a: K, b: K) -> K,
    ) -> Result<Vector<K>, String> {
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
    pub fn add(&mut self, b: &Vector<K>) -> Result<(), String> {
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
    pub fn sub(&mut self, b: &Vector<K>) -> Result<(), String> {
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
    pub fn scl(&mut self, value: K) {
        let size = self.size();
        for column in 0..size {
            self[column] *= value
        }
    }

    #[allow(dead_code)]
    pub fn dot(&self, b: Vector<K>) -> K {
        let size = self.size();
        if size != b.size() {
            return K::default();
        }

        let mut result = K::default();
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
