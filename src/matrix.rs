use std::{
    fmt::{self, Debug},
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
    slice::Iter,
};

use crate::{linear_interpolation::Lerp, norm::Norm, vector::Vector};

#[derive(Debug)]
pub struct Matrix<
    K: Default
        + Clone
        + Copy
        + Debug
        + PartialEq
        + AddAssign
        + SubAssign
        + MulAssign
        + Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + Div<Output = K>
        + Mul<f64, Output = K>
        + Norm,
> {
    elements: Vec<Vec<K>>,
}

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > fmt::Display for Matrix<K>
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
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Default for Matrix<K>
{
    fn default() -> Self {
        Self::new([0, 0])
    }
}

// * Clone

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Clone for Matrix<K>
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

// * Operations

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Add for Matrix<K>
{
    type Output = Matrix<K>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.shape() == rhs.shape() {
            let mut matrix = Matrix::new(self.shape());
            for x in 0..self.shape()[0] {
                for y in 0..self.shape()[1] {
                    matrix[x][y] = self[x][y] + rhs[x][y];
                }
            }
            matrix
        } else {
            Matrix::new([0, 0])
        }
    }
}

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Sub for Matrix<K>
{
    type Output = Matrix<K>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.shape() == rhs.shape() {
            let mut matrix = Matrix::new(self.shape());
            for x in 0..self.shape()[0] {
                for y in 0..self.shape()[1] {
                    matrix[x][y] = self[x][y] - rhs[x][y];
                }
            }
            matrix
        } else {
            Matrix::new([0, 0])
        }
    }
}

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Mul<K> for Matrix<K>
{
    type Output = Matrix<K>;

    fn mul(self, rhs: K) -> Self::Output {
        let mut matrix = Matrix::new(self.shape());
        for x in 0..self.shape()[0] {
            for y in 0..self.shape()[1] {
                matrix[x][y] = self[x][y] * rhs;
            }
        }
        matrix
    }
}

// *> From

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > From<Vec<K>> for Matrix<K>
{
    fn from(vec: Vec<K>) -> Self {
        Matrix {
            elements: vec![vec],
        }
    }
}

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > From<Vec<Vec<K>>> for Matrix<K>
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

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
        const N: usize,
    > From<[K; N]> for Matrix<K>
{
    fn from(slice: [K; N]) -> Self {
        Matrix {
            elements: vec![slice.to_vec()],
        }
    }
}

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
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

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Index<usize> for Matrix<K>
{
    type Output = Vec<K>;

    fn index(&self, i: usize) -> &Vec<K> {
        &self.elements[i]
    }
}

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > IndexMut<usize> for Matrix<K>
{
    fn index_mut(&mut self, i: usize) -> &mut Vec<K> {
        &mut self.elements[i]
    }
}

// *< From

// *> Iterator

pub struct ColumnIterator<
    'a,
    K: Default
        + Clone
        + Copy
        + Debug
        + PartialEq
        + AddAssign
        + SubAssign
        + MulAssign
        + Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + Div<Output = K>
        + Mul<f64, Output = K>
        + Norm,
> {
    matrix: &'a Matrix<K>,
    shape: [usize; 2],
    current_row: usize,
    current_column: usize,
}

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Iterator for ColumnIterator<'_, K>
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

        Some(self.matrix[row][column])
    }
}

pub struct TupleIterator<
    'a,
    K: Default
        + Clone
        + Copy
        + Debug
        + PartialEq
        + AddAssign
        + SubAssign
        + MulAssign
        + Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + Div<Output = K>
        + Mul<f64, Output = K>
        + Norm,
> {
    matrix_a: &'a Matrix<K>,
    matrix_b: &'a Matrix<K>,
    shape: [usize; 2],
    current_row: usize,
    current_column: usize,
}

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Iterator for TupleIterator<'_, K>
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

        Some([self.matrix_a[row][column], self.matrix_b[row][column]])
    }
}

// *< Iterator

// * Lerp

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Lerp for Matrix<K>
{
    fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        // if !(0. ..=1.).contains(&t) {
        //     return Err(format!(
        //         "Invalid t value of {}, should be between 0. and 1.",
        //         t
        //     ));
        // }

        if a.shape() != b.shape() {
            return Matrix::new([0, 0]);
        }

        let mut result = Matrix::new(a.shape());
        for x in 0..a.shape()[0] {
            for y in 0..a.shape()[1] {
                result[x][y] = a[x][y] * (1. - t) + b[x][y] * t;
            }
        }
        result
    }
}

// * Matrix

impl<
        K: Default
            + Clone
            + Copy
            + Debug
            + PartialEq
            + AddAssign
            + SubAssign
            + MulAssign
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Div<Output = K>
            + Mul<f64, Output = K>
            + Norm,
    > Matrix<K>
{
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
            matrix[diagonal][diagonal] = value
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
                new_matrix[row][column] = callback(a[row][column], b[row][column]);
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
                self[row][column] += b[row][column];
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
                self[row][column] -= b[row][column];
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn scl(&mut self, value: K) {
        let shape = self.shape();
        for row in 0..shape[0] {
            for column in 0..shape[1] {
                self[row][column] *= value;
            }
        }
    }

    pub fn mul_vec(&self, vector: &Vector<K>) -> Vector<K> {
        let shape = self.shape();
        if shape[1] != vector.size() {
            return Vector::new(0);
        }

        let mut result = Vector::new(shape[0]);
        for row in 0..shape[0] {
            let mut value = K::default();
            for column in 0..shape[1] {
                value += self[row][column] * vector[column];
            }
            result[row] = value;
        }

        result
    }

    pub fn mul_mat(&self, matrix: &Matrix<K>) -> Matrix<K> {
        let self_shape = self.shape();
        let other_shape = matrix.shape();
        if self_shape[0] != other_shape[1] || self_shape[1] != other_shape[0] {
            return Matrix::new([0, 0]);
        }

        let mut result = Matrix::new([self_shape[0], other_shape[1]]);
        for row in 0..self_shape[0] {
            for result_column in 0..other_shape[1] {
                let mut value = K::default();
                for column in 0..self_shape[1] {
                    value += self[row][column] * matrix[column][result_column];
                }
                result[row][result_column] = value;
            }
        }

        result
    }

    pub fn trace(&self) -> K {
        let shape = self.shape();
        if shape[0] != shape[1] {
            return K::default();
        }

        let mut result = K::default();
        for i in 0..shape[0] {
            result = result + self[i][i];
        }

        result
    }

    pub fn transpose(&self) -> Matrix<K> {
        let shape = self.shape();
        let mut result = Matrix::new([shape[1], shape[0]]);
        for row in 0..shape[0] {
            for column in 0..shape[1] {
                result[column][row] = self[row][column];
            }
        }
        result
    }

    pub fn row_echelon(&self) -> Matrix<K> {
        let shape = self.shape();
        let zero_value = K::default();
        let mut lead = 0;

        let mut result = Matrix::clone(self);
        for r in 0..shape[0] {
            if shape[1] <= lead {
                return result;
            }
            let mut i = r;
            while result[i][lead] == zero_value {
                i += 1;
                if shape[0] == i {
                    i = r;
                    lead += 1;
                    if shape[1] == lead {
                        return result;
                    }
                }
            }

            let tmp = result[i].clone();
            result[i] = result[r].clone();
            result[r] = tmp;

            let val = result[r][lead];
            for j in 0..shape[1] {
                result[r][j] = result[r][j] / val;
            }

            for i in 0..shape[0] {
                if i == r {
                    continue;
                }
                let val = result[i][lead];
                for j in 0..shape[1] {
                    result[i][j] = result[i][j] - val * result[r][j];
                }
            }

            lead += 1;
        }

        result
    }

    fn determinant_square(a: K, b: K, c: K, d: K) -> K {
        return a * d - b * c;
    }

    fn determinant_cube(values: [[K; 3]; 3]) -> K {
        return values[0][0]
            * Matrix::determinant_square(values[1][1], values[1][2], values[2][1], values[2][2])
            - values[0][1]
                * Matrix::determinant_square(
                    values[1][0],
                    values[2][0],
                    values[1][2],
                    values[2][2],
                )
            + values[0][2]
                * Matrix::determinant_square(
                    values[1][0],
                    values[2][0],
                    values[1][1],
                    values[2][1],
                );
    }

    pub fn determinant(&self) -> K {
        let shape = self.shape();
        if shape[0] != shape[1] || shape[0] < 2 || shape[0] > 4 {
            return K::default();
        }

        if shape[0] == 2 {
            return Matrix::determinant_square(self[0][0], self[0][1], self[1][0], self[1][1]);
        } else if shape[0] == 3 {
            return Matrix::determinant_cube([
                [self[0][0], self[0][1], self[0][2]],
                [self[1][0], self[1][1], self[1][2]],
                [self[2][0], self[2][1], self[2][2]],
            ]);
        }

        return self[0][0]
            * Matrix::determinant_cube([
                [self[1][1], self[1][2], self[1][3]],
                [self[2][1], self[2][2], self[2][3]],
                [self[3][1], self[3][2], self[3][3]],
            ])
            - self[0][1]
                * Matrix::determinant_cube([
                    [self[1][0], self[1][2], self[1][3]],
                    [self[2][0], self[2][2], self[2][3]],
                    [self[3][0], self[3][2], self[3][3]],
                ])
            + self[0][2]
                * Matrix::determinant_cube([
                    [self[1][0], self[1][1], self[1][3]],
                    [self[2][0], self[2][1], self[2][3]],
                    [self[3][0], self[3][1], self[3][3]],
                ])
            - self[0][3]
                * Matrix::determinant_cube([
                    [self[1][0], self[1][1], self[1][2]],
                    [self[2][0], self[2][1], self[2][2]],
                    [self[3][0], self[3][1], self[3][2]],
                ]);
    }

    pub fn rank(&self) -> usize {
        let shape = self.shape();

        // * Calculate the row echelon form (not reduced) with gaussian elimination
        let mut reduced = self.clone();
        let mut h = 0;
        let mut k = 0;
        while h < shape[0] && k < shape[1] {
            let mut i_max = h;
            for i in (h + 1)..shape[0] {
                if reduced[i][k].abs() > reduced[i_max][k].abs() {
                    i_max = i;
                }
            }
            if reduced[i_max][k] == K::default() {
                k += 1;
            } else {
                let tmp = reduced[h].clone();
                reduced[h] = reduced[i_max].clone();
                reduced[i_max] = tmp;
                for i in (h + 1)..shape[0] {
                    let f = reduced[i][k] / reduced[h][k];
                    reduced[i][k] = K::default();
                    for j in (k + 1)..shape[1] {
                        reduced[i][j] = reduced[i][j] - reduced[h][j] * f;
                    }
                }
                h += 1;
                k += 1;
            }
        }

        // * The rank is the number of non-zero rows
        for r in 0..shape[0] {
            if reduced[r].iter().find(|&&c| c != K::default()).is_none() {
                return r;
            }
        }

        shape[0]
    }
}
