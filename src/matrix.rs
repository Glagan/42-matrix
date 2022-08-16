use std::{
    fmt::{self, Debug},
    ops::{AddAssign, MulAssign, SubAssign},
    slice::Iter,
};

#[derive(Debug)]
pub struct Matrix<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign> {
    elements: Vec<Vec<K>>,
}

impl<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign> fmt::Display
    for Matrix<K>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}

impl<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign> Default for Matrix<K> {
    fn default() -> Self {
        Self::new([0, 0])
    }
}

// *> From

impl<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign> From<Vec<K>>
    for Matrix<K>
{
    fn from(vec: Vec<K>) -> Self {
        Matrix {
            elements: vec![vec],
        }
    }
}

impl<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign> From<Vec<Vec<K>>>
    for Matrix<K>
{
    fn from(mat: Vec<Vec<K>>) -> Matrix<K> {
        // let length = mat.len();
        // if length == 0 || mat[0].len() == length {
        //     return Ok(Matrix { elements: mat });
        // }
        // Err(())
        Matrix { elements: mat }
    }
}

impl<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign, const N: usize>
    From<[K; N]> for Matrix<K>
{
    fn from(slice: [K; N]) -> Self {
        Matrix {
            elements: vec![slice.to_vec()],
        }
    }
}

impl<
        K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign,
        const N: usize,
        const M: usize,
    > From<[[K; N]; M]> for Matrix<K>
{
    fn from(mat: [[K; N]; M]) -> Self {
        Matrix {
            elements: mat.iter().map(|slice| slice.to_vec()).collect(),
        }
    }
}

// *< From

// *> Iterator

pub struct ColumnIterator<'a, K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign>
{
    matrix: &'a Matrix<K>,
    shape: [usize; 2],
    current_row: usize,
    current_column: usize,
}

impl<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign> Iterator
    for ColumnIterator<'_, K>
{
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.shape[0] == 0 || self.shape[1] == 0 {
            return None;
        }
        if self.current_column >= self.shape[1] {
            return None;
        }

        let row = self.current_row;
        let column = self.current_column;

        if row + 1 >= self.shape[0] {
            self.current_column += 1;
            self.current_row = 0;
        } else {
            self.current_row += 1;
        }

        Some(self.matrix.elements[row][column])
    }
}

pub struct TupleIterator<'a, K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign>
{
    matrix_a: &'a Matrix<K>,
    matrix_b: &'a Matrix<K>,
    shape: [usize; 2],
    current_row: usize,
    current_column: usize,
}

impl<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign> Iterator
    for TupleIterator<'_, K>
{
    type Item = [K; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if self.shape[0] == 0 || self.shape[1] == 0 {
            return None;
        }
        if self.current_row >= self.shape[1] {
            return None;
        }

        let row = self.current_row;
        let column = self.current_column;

        if column + 1 >= self.shape[1] {
            self.current_row += 1;
            self.current_column = 0;
        } else {
            self.current_column += 1;
        }

        Some([
            self.matrix_a.elements[row][column],
            self.matrix_b.elements[row][column],
        ])
    }
}

// *< Iterator

impl<K: Default + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign> Matrix<K> {
    pub fn new(shape: [usize; 2]) -> Matrix<K> {
        Matrix {
            elements: vec![vec![K::default(); shape[1]]; shape[0]],
        }
    }

    // * Utility functions

    // Return the identity matrix of the given size
    #[allow(dead_code)]
    pub fn identity(size: usize, value: K) -> Matrix<K> {
        if size == 0 {
            return Matrix::default();
        }
        let mut matrix = Matrix {
            elements: vec![vec![K::default(); size]; size],
        };
        for diagonal in 0..size {
            matrix.elements[diagonal][diagonal] = value
        }
        matrix
    }

    // Shape of the matrix, as [rows, columns]
    #[allow(dead_code)]
    pub fn shape(&self) -> [usize; 2] {
        let width = self.elements.len();
        if width > 0 {
            return [self.elements.len(), self.elements[0].len()];
        }
        [0, 0]
    }

    #[allow(dead_code)]
    pub fn all(&self) -> &Vec<Vec<K>> {
        &self.elements
    }

    // Fill the matrix with a given value
    #[allow(dead_code)]
    pub fn fill(&mut self, value: K) {
        for row in self.elements.iter_mut() {
            for element in row {
                *element = value
            }
        }
    }

    // Create an iterator in the direction of the rows of the matrix
    #[allow(dead_code)]
    pub fn iter_rows(&self) -> Iter<'_, Vec<K>> {
        self.elements.iter()
    }

    // Create an iterator in the direction of the columns of the matrix
    #[allow(dead_code)]
    pub fn iter_cols(&self) -> ColumnIterator<K> {
        ColumnIterator {
            matrix: self,
            shape: self.shape(),
            current_column: 0,
            current_row: 0,
        }
    }

    // Create an iterator with the value of two matrices
    #[allow(dead_code)]
    pub fn iter_tuple<'a>(
        a: &'a Matrix<K>,
        b: &'a Matrix<K>,
    ) -> Result<TupleIterator<'a, K>, String> {
        let a_shape = a.shape();
        if a_shape != b.shape() {
            return Err(format!("Invalid shapes {:?} and {:?}", a_shape, b.shape()));
        }

        Ok(TupleIterator {
            matrix_a: a,
            matrix_b: b,
            shape: a_shape,
            current_row: 0,
            current_column: 0,
        })
    }

    // Create a TupleIterator with another matrix
    #[allow(dead_code)]
    pub fn iter_with<'a>(&'a self, b: &'a Matrix<K>) -> Result<TupleIterator<'a, K>, String> {
        Matrix::iter_tuple(self, b)
    }

    // Apply a function on each of the elements of the matrix and return a new matrix with the function applied
    #[allow(dead_code)]
    pub fn map<'a>(
        a: &'a Matrix<K>,
        b: &'a Matrix<K>,
        callback: fn(a: K, b: K) -> K,
    ) -> Result<Matrix<K>, String> {
        let a_shape = a.shape();
        if a_shape != b.shape() {
            return Err(format!("Invalid shapes {:?} and {:?}", a_shape, b.shape()));
        }

        let mut new_matrix = Matrix::new(a_shape);
        for row in 0..a_shape[0] {
            for column in 0..a_shape[1] {
                new_matrix.elements[row][column] =
                    callback(a.elements[row][column], b.elements[row][column]);
            }
        }

        Ok(new_matrix)
    }

    // * Subject functions

    #[allow(dead_code)]
    pub fn add(&mut self, b: &Matrix<K>) -> Result<(), String> {
        let shape = self.shape();
        if shape != b.shape() {
            return Err(format!("Invalid shapes {:?} and {:?}", shape, b.shape()));
        }

        for row in 0..shape[0] {
            for column in 0..shape[1] {
                self.elements[row][column] += b.elements[row][column];
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn sub(&mut self, b: &Matrix<K>) -> Result<(), String> {
        let shape = self.shape();
        if shape != b.shape() {
            return Err(format!("Invalid shapes {:?} and {:?}", shape, b.shape()));
        }

        for row in 0..shape[0] {
            for column in 0..shape[1] {
                self.elements[row][column] -= b.elements[row][column];
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn scl(&mut self, value: K) {
        let shape = self.shape();
        for row in 0..shape[0] {
            for column in 0..shape[1] {
                self.elements[row][column] *= value;
            }
        }
    }
}
