use std::{
    f64::consts::PI,
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
        if a.shape() != b.shape() {
            return Matrix::new([0, 0]);
        }

        let [rows, cols] = a.shape();
        let mut result = Matrix::new(a.shape());
        for x in 0..rows {
            for y in 0..cols {
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

    pub fn add(&mut self, b: &Matrix) {
        let shape = self.shape();
        if shape != b.shape() {
            // return Err(format!("Invalid shapes {:?} and {:?}", shape, b.shape()));
            return;
        }

        for row in 0..shape[0] {
            for column in 0..shape[1] {
                self[row][column] += b[row][column];
            }
        }
    }

    pub fn sub(&mut self, b: &Matrix) {
        let shape = self.shape();
        if shape != b.shape() {
            // return Err(format!("Invalid shapes {:?} and {:?}", shape, b.shape()));
            return;
        }

        for row in 0..shape[0] {
            for column in 0..shape[1] {
                self[row][column] -= b[row][column];
            }
        }
    }

    pub fn scl(&mut self, value: f64) {
        let [rows, cols] = self.shape();
        for row in 0..rows {
            for column in 0..cols {
                self[row][column] *= value;
            }
        }
    }

    pub fn mul_vec(&self, vector: &Vector) -> Vector {
        let [rows, cols] = self.shape();
        if cols != vector.size() {
            return Vector::new(0);
        }

        let mut result = Vector::new(rows);
        for row in 0..rows {
            let mut value = 0.;
            for column in 0..cols {
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
        let [rows, cols] = self.shape();
        if rows != cols {
            return 0.;
        }

        let mut result = 0.;
        for i in 0..rows {
            result += self[i][i];
        }

        result
    }

    pub fn transpose(&self) -> Matrix {
        let [rows, cols] = self.shape();
        let mut result = Matrix::new([cols, rows]);
        for row in 0..rows {
            for column in 0..cols {
                result[column][row] = self[row][column];
            }
        }
        result
    }

    pub fn row_echelon(&self) -> Matrix {
        let [rows, cols] = self.shape();
        let mut lead = 0;

        let mut result = Matrix::clone(self);
        for r in 0..rows {
            if cols <= lead {
                return result;
            }
            let mut i = r;
            while result[i][lead] == 0. {
                i += 1;
                if rows == i {
                    i = r;
                    lead += 1;
                    if cols == lead {
                        return result;
                    }
                }
            }

            result.elements.swap(i, r);

            let val = result[r][lead];
            for j in 0..cols {
                result[r][j] /= val;
            }

            for i in 0..rows {
                if i == r {
                    continue;
                }
                let val = result[i][lead];
                for j in 0..cols {
                    result[i][j] -= val * result[r][j];
                }
            }

            lead += 1;
        }

        result
    }

    pub fn reduced_row_echelon(&self) -> Matrix {
        self.row_echelon()
    }

    // Use the Bareiss algorithm to find the determinant
    pub fn determinant(&self) -> f64 {
        let [rows, cols] = self.shape();
        if rows != cols {
            return 0.;
        }

        if rows == 1 {
            return self[0][0];
        }

        let mut sign = 1.;
        let mut matrix = self.clone();
        for k in 0..(rows - 1) {
            // Pivot row swap if needed
            if matrix[k][k] == 0. {
                let mut m = k + 1;
                while m < rows {
                    if matrix[m][k] != 0. {
                        matrix.elements.swap(m, k);
                        sign = -sign;
                        break;
                    }
                    m += 1;
                }
                if m == rows {
                    return 0.;
                }
            }

            // Formula
            for i in (k + 1)..rows {
                for j in (k + 1)..cols {
                    matrix[i][j] = matrix[k][k] * matrix[i][j] - matrix[i][k] * matrix[k][j];
                    if k != 0 {
                        matrix[i][j] /= matrix[k - 1][k - 1];
                    }
                }
            }
        }

        sign * matrix[rows - 1][rows - 1]
    }

    pub fn inverse(&self) -> Result<Matrix, String> {
        let [rows, cols] = self.shape();
        if rows != cols {
            return Ok(Matrix::new([0, 0]));
        }

        if rows < 1 {
            return Ok(Matrix::new([0, 0]));
        } else if rows < 2 {
            return Ok(Matrix::clone(self));
        }

        let mut lead = 0;

        // * Calculate the reduced row echelon form
        // * -- while updating the augmented matrix
        let mut reduced = Matrix::clone(self);
        let mut result = Matrix::identity(rows, 1.);
        for r in 0..rows {
            if cols <= lead {
                return Err("Singular matrix".to_string());
            }
            let mut i = r;
            while reduced[i][lead] == 0. {
                i += 1;
                if rows == i {
                    i = r;
                    lead += 1;
                    if cols == lead {
                        return Err("Singular matrix".to_string());
                    }
                }
            }

            reduced.elements.swap(i, r);
            result.elements.swap(i, r);

            let val = reduced[r][lead];
            for j in 0..cols {
                reduced[r][j] /= val;
                result[r][j] /= val;
            }

            for i in 0..rows {
                if i == r {
                    continue;
                }
                let val = reduced[i][lead];
                for j in 0..cols {
                    reduced[i][j] -= val * reduced[r][j];
                    result[i][j] -= val * result[r][j];
                }
            }

            lead += 1;
        }

        Ok(result)
    }

    pub fn rank(&self) -> usize {
        let [rows, cols] = self.shape();

        // * Calculate the row echelon form (not reduced) with gaussian elimination
        let mut reduced = self.clone();
        let mut h = 0;
        let mut k = 0;
        while h < rows && k < cols {
            let mut i_max = h;
            for i in (h + 1)..rows {
                if reduced[i][k].abs() > reduced[i_max][k].abs() {
                    i_max = i;
                }
            }
            if reduced[i_max][k] == 0. {
                k += 1;
            } else {
                reduced.elements.swap(h, i_max);
                for i in (h + 1)..rows {
                    let f = reduced[i][k] / reduced[h][k];
                    reduced[i][k] = 0.;
                    for j in (k + 1)..cols {
                        reduced[i][j] -= reduced[h][j] * f;
                    }
                }
                h += 1;
                k += 1;
            }
        }

        // * The rank is the number of non-zero rows
        // -- which is already in `h`
        h
    }

    pub fn projection(fov: f64, ratio: f64, near: f64, far: f64) -> Matrix {
        let s = 1. / (f64::tan((fov / 2.) * (PI / 180.)));
        let x_scale = if ratio < 1. { ratio } else { 1. };
        let y_scale = if ratio > 1. { 1. / ratio } else { 1. };
        Matrix::from([
            [s * x_scale, 0., 0., 0.],
            [0., s * y_scale, 0., 0.],
            [0., 0., -far / (far - near), -1.],
            [0., 0., -((far * near) / (far - near)), 0.],
        ])
    }
}
