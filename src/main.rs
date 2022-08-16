use crate::{matrix::Matrix, vector::Vector};

pub mod linear_combination;
pub mod matrix;
pub mod vector;

fn main() {
    // *

    let vec: Vector<f32> = Vector::new(4);
    println!("vec: {} (shape: {:#?})", vec, vec.shape());
    let vec: Vector<f32> = Vector::from(vec![1., 2., 3., 4.]);
    println!("vec: {} (shape: {:#?})", vec, vec.shape());
    let vec: Vector<f32> = Vector::from([1., 2., 3., 4.]);
    println!("vec: {} (shape: {:#?})", vec, vec.shape());
    let vec: Vector<f32> = Vector::from([1., 2., 3., 4.]);
    println!(
        "vec-to-mat: {} (shape: {:#?})",
        vec.reshape(),
        vec.reshape().shape()
    );

    // *

    let matrix: Matrix<f32> = Matrix::new([4, 4]);
    println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());
    let matrix: Matrix<f32> = Matrix::from(vec![1., 2., 3., 4., 5., 6.]);
    println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());
    let matrix: Matrix<f32> = Matrix::from(vec![vec![1., 2., 3.], vec![4., 5., 6.]]);
    println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());
    let matrix: Matrix<f32> = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());
    let matrix: Matrix<f32> = Matrix::identity(4, 1.);
    println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());

    for column in matrix.iter_cols() {
        println!("[{}]", column);
    }

    // *

    let matrix_a: Matrix<f32> = Matrix::identity(4, 1.);
    let matrix_b: Matrix<f32> = Matrix::identity(4, 1.);
    let matrix_c = Matrix::map(&matrix_a, &matrix_b, |a, b| a + b);
    if let Ok(matrix) = matrix_c {
        println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());
    } else {
        println!("failed to map matrix_a and matrix_b");
    }
}
