use crate::matrix::Matrix;
use std::{
    fmt::{self, Debug},
    ops::{AddAssign, MulAssign, SubAssign},
    slice::Iter,
};

#[derive(Debug)]
pub struct Vector<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign> {
    elements: Vec<K>,
}

impl<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign> fmt::Display
    for Vector<K>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}

impl<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign> Default for Vector<K> {
    fn default() -> Self {
        Self::new(0)
    }
}

// *> From

impl<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign> From<Vec<K>>
    for Vector<K>
{
    fn from(vec: Vec<K>) -> Self {
        Vector { elements: vec }
    }
}

impl<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign, const N: usize>
    From<[K; N]> for Vector<K>
{
    fn from(slice: [K; N]) -> Self {
        Vector {
            elements: slice.to_vec(),
        }
    }
}

// *< From

// *> Iterator

pub struct TupleIterator<'a, K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign>
{
    vector_a: &'a Vector<K>,
    vector_b: &'a Vector<K>,
    size: usize,
    current_column: usize,
}

impl<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign> Iterator
    for TupleIterator<'_, K>
{
    type Item = [K; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }
        if self.current_column >= self.size {
            return None;
        }

        let column = self.current_column;
        self.current_column += 1;
        Some([
            self.vector_a.elements[column],
            self.vector_b.elements[column],
        ])
    }
}

// *< Iterator

impl<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign> Vector<K> {
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

    // Create an iterator with the value of two vectors
    #[allow(dead_code)]
    pub fn iter_tuple<'a>(
        a: &'a Vector<K>,
        b: &'a Vector<K>,
    ) -> Result<TupleIterator<'a, K>, String> {
        let a_size = a.size();
        if a_size != b.size() {
            return Err(format!("Invalid sizes {:?} and {:?}", a_size, b.size()));
        }

        Ok(TupleIterator {
            vector_a: a,
            vector_b: b,
            size: a_size,
            current_column: 0,
        })
    }

    // Create a TupleIterator with another vector
    #[allow(dead_code)]
    pub fn iter_with<'a>(&'a self, b: &'a Vector<K>) -> Result<TupleIterator<'a, K>, String> {
        Vector::iter_tuple(self, b)
    }

    // Apply a function on each of the elements of the vector and return a new vector with the function applied
    #[allow(dead_code)]
    pub fn map<'a>(&'a self, callback: fn(value: K) -> K) -> Result<Vector<K>, String> {
        let size = self.size();
        let mut new_vector = Vector::new(size);
        for column in 0..size {
            new_vector.elements[column] = callback(self.elements[column]);
        }

        Ok(new_vector)
    }

    // Apply a function on each of the elements of two vectors and return a new vector with the function applied
    #[allow(dead_code)]
    pub fn map_tuple<'a>(
        a: &'a Vector<K>,
        b: &'a Vector<K>,
        callback: fn(a: K, b: K) -> K,
    ) -> Result<Vector<K>, String> {
        let a_size = a.size();
        if a_size != b.size() {
            return Err(format!("Invalid sizes {:?} and {:?}", a_size, b.size()));
        }

        let mut new_vector = Vector::new(a_size);
        for column in 0..a_size {
            new_vector.elements[column] = callback(a.elements[column], b.elements[column]);
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
            self.elements[column] += b.elements[column]
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
            self.elements[column] -= b.elements[column]
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn scl(&mut self, value: K) {
        let size = self.size();
        for column in 0..size {
            self.elements[column] *= value
        }
    }
}
