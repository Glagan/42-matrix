use crate::{linear_interpolation::lerp, matrix::Matrix, vector::Vector};

pub mod linear_combination;
pub mod linear_interpolation;
pub mod matrix;
pub mod vector;

fn main() {
    // *

    let vec: Vector<f64> = Vector::new(4);
    println!("vec: {} (shape: {:#?})", vec, vec.shape());
    let vec: Vector<f64> = Vector::from(vec![1., 2., 3., 4.]);
    println!("vec: {} (shape: {:#?})", vec, vec.shape());
    let vec: Vector<f64> = Vector::from([1., 2., 3., 4.]);
    println!("vec: {} (shape: {:#?})", vec, vec.shape());
    let vec: Vector<f64> = Vector::from([1., 2., 3., 4.]);
    println!(
        "vec-to-mat: {} (shape: {:#?})",
        vec.reshape(),
        vec.reshape().shape()
    );

    // *

    let matrix: Matrix<f64> = Matrix::new([4, 4]);
    println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());
    let matrix: Matrix<f64> = Matrix::from(vec![1., 2., 3., 4., 5., 6.]);
    println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());
    let matrix: Matrix<f64> = Matrix::from(vec![vec![1., 2., 3.], vec![4., 5., 6.]]);
    println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());
    let matrix: Matrix<f64> = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());
    let matrix: Matrix<f64> = Matrix::identity(4, 1.);
    println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());

    for column in matrix.iter_cols() {
        println!("[{}]", column);
    }

    // *

    let matrix_a: Matrix<f64> = Matrix::identity(4, 1.);
    let matrix_b: Matrix<f64> = Matrix::identity(4, 1.);
    let matrix_c = Matrix::map(&matrix_a, &matrix_b, |a, b| a + b);
    if let Ok(matrix) = matrix_c {
        println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());
    } else {
        println!("failed to map matrix_a and matrix_b");
    }

    // *

    println!("{} (0.0)", lerp(&0., &1., 0.));
    println!("{} (1.0)", lerp(&0., &1., 1.));
    println!("{} (0.5)", lerp(&0., &1., 0.5));
    println!("{} (27.3)", lerp(&21., &42., 0.3));
    println!(
        "{} ([2.6, 1.3])",
        lerp(&Vector::from([2., 1.]), &Vector::from([4., 2.]), 0.3)
    );
    println!(
        "{} ([[11.0, 5.5], [16.5, 22.0]])",
        lerp(
            &Matrix::from([[2., 1.], [3., 4.]]),
            &Matrix::from([[20., 10.], [30., 40.]]),
            0.5
        )
    );
}
