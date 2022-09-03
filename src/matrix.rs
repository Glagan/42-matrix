use std::{
    fmt::{self, Debug},
    ops::{Add, Index, IndexMut, Mul, Sub},
    slice::Iter,
};

use crate::{linear_interpolation::Lerp, vector::Vector};

#[derive(Debug)]
pub struct Matrix {
    elements: Vec<Vec<f64>>,
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new([0, 0])
    }
}

// * Clone

impl Clone for Matrix {
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

impl Add for Matrix {
    type Output = Matrix;

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

impl Sub for Matrix {
    type Output = Matrix;

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

impl Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
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

impl From<Vec<f64>> for Matrix {
    fn from(vec: Vec<f64>) -> Self {
        Matrix {
            elements: vec![vec],
        }
    }
}

impl From<Vec<Vec<f64>>> for Matrix {
    fn from(mat: Vec<Vec<f64>>) -> Matrix {
        // let length = mat.len();
        // if length == 0 || mat[0].len() == length {
        //     return Ok(Matrix { elements: mat });
        // }
        // Err(())
        Matrix { elements: mat }
    }
}

impl<const N: usize> From<[f64; N]> for Matrix {
    fn from(slice: [f64; N]) -> Self {
        Matrix {
            elements: vec![slice.to_vec()],
        }
    }
}

impl<const N: usize, const M: usize> From<[[f64; N]; M]> for Matrix {
    fn from(mat: [[f64; N]; M]) -> Self {
        Matrix {
            elements: mat.iter().map(|slice| slice.to_vec()).collect(),
        }
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, i: usize) -> &Vec<f64> {
        &self.elements[i]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, i: usize) -> &mut Vec<f64> {
        &mut self.elements[i]
    }
}

// *< From

// *> Iterator

pub struct ColumnIterator<'a> {
    matrix: &'a Matrix,
    shape: [usize; 2],
    current_row: usize,
    current_column: usize,
}

impl Iterator for ColumnIterator<'_> {
    type Item = f64;

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

pub struct TupleIterator<'a> {
    matrix_a: &'a Matrix,
    matrix_b: &'a Matrix,
    shape: [usize; 2],
    current_row: usize,
    current_column: usize,
}

impl Iterator for TupleIterator<'_> {
    type Item = [f64; 2];

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

impl Lerp for Matrix {
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

impl Matrix {
    pub fn new(shape: [usize; 2]) -> Matrix {
        Matrix {
            elements: vec![vec![0.; shape[1]]; shape[0]],
        }
    }

    // * Utility functions

    // Return the identity matrix of the given size
    #[allow(dead_code)]
    pub fn identity(size: usize, value: f64) -> Matrix {
        if size == 0 {
            return Matrix::default();
        }
        let mut matrix = Matrix {
            elements: vec![vec![0.; size]; size],
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
    pub fn all(&self) -> &Vec<Vec<f64>> {
        &self.elements
    }

    // Create an iterator in the direction of the rows of the matrix
    #[allow(dead_code)]
    pub fn iter_rows(&self) -> Iter<'_, Vec<f64>> {
        self.elements.iter()
    }

    // Create an iterator in the direction of the columns of the matrix
    #[allow(dead_code)]
    pub fn iter_cols(&self) -> ColumnIterator {
        ColumnIterator {
            matrix: self,
            shape: self.shape(),
            current_column: 0,
            current_row: 0,
        }
    }

    // Create an iterator with the value of two matrices
    #[allow(dead_code)]
    pub fn iter_tuple<'a>(a: &'a Matrix, b: &'a Matrix) -> Result<TupleIterator<'a>, String> {
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
    pub fn iter_with<'a>(&'a self, b: &'a Matrix) -> Result<TupleIterator<'a>, String> {
        Matrix::iter_tuple(self, b)
    }

    // Apply a function on each of the elements of the matrix and return a new matrix with the function applied
    #[allow(dead_code)]
    pub fn map<'a>(
        a: &'a Matrix,
        b: &'a Matrix,
        callback: fn(a: f64, b: f64) -> f64,
    ) -> Result<Matrix, String> {
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
    pub fn add(&mut self, b: &Matrix) -> Result<(), String> {
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
    pub fn sub(&mut self, b: &Matrix) -> Result<(), String> {
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
    pub fn scl(&mut self, value: f64) {
        let shape = self.shape();
        for row in 0..shape[0] {
            for column in 0..shape[1] {
                self[row][column] *= value;
            }
        }
    }

    pub fn mul_vec(&self, vector: &Vector) -> Vector {
        let shape = self.shape();
        if shape[1] != vector.size() {
            return Vector::new(0);
        }

        let mut result = Vector::new(shape[0]);
        for row in 0..shape[0] {
            let mut value = 0.;
            for column in 0..shape[1] {
                value += self[row][column] * vector[column];
            }
            result[row] = value;
        }

        result
    }

    pub fn mul_mat(&self, matrix: &Matrix) -> Matrix {
        let self_shape = self.shape();
        let other_shape = matrix.shape();
        if self_shape[0] != other_shape[1] || self_shape[1] != other_shape[0] {
            return Matrix::new([0, 0]);
        }

        let mut result = Matrix::new([self_shape[0], other_shape[1]]);
        for row in 0..self_shape[0] {
            for result_column in 0..other_shape[1] {
                let mut value = 0.;
                for column in 0..self_shape[1] {
                    value += self[row][column] * matrix[column][result_column];
                }
                result[row][result_column] = value;
            }
        }

        result
    }

    pub fn trace(&self) -> f64 {
        let shape = self.shape();
        if shape[0] != shape[1] {
            return 0.;
        }

        let mut result = 0.;
        for i in 0..shape[0] {
            result = result + self[i][i];
        }

        result
    }

    pub fn transpose(&self) -> Matrix {
        let shape = self.shape();
        let mut result = Matrix::new([shape[1], shape[0]]);
        for row in 0..shape[0] {
            for column in 0..shape[1] {
                result[column][row] = self[row][column];
            }
        }
        result
    }

    pub fn row_echelon(&self) -> Matrix {
        let shape = self.shape();
        let mut lead = 0;

        let mut result = Matrix::clone(self);
        for r in 0..shape[0] {
            if shape[1] <= lead {
                return result;
            }
            let mut i = r;
            while result[i][lead] == 0. {
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

    pub fn reduced_row_echelon(&self) -> Matrix {
        self.row_echelon()
    }

    fn determinant_square(a: f64, b: f64, c: f64, d: f64) -> f64 {
        return a * d - b * c;
    }

    fn determinant_cube(values: [[f64; 3]; 3]) -> f64 {
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

    pub fn determinant(&self) -> f64 {
        let shape = self.shape();
        if shape[0] != shape[1] || shape[0] < 2 || shape[0] > 4 {
            return 0.;
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
            if reduced[i_max][k] == 0. {
                k += 1;
            } else {
                let tmp = reduced[h].clone();
                reduced[h] = reduced[i_max].clone();
                reduced[i_max] = tmp;
                for i in (h + 1)..shape[0] {
                    let f = reduced[i][k] / reduced[h][k];
                    reduced[i][k] = 0.;
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
            if reduced[r].iter().find(|&&c| c != 0.).is_none() {
                return r;
            }
        }

        shape[0]
    }
}
