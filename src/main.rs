use crate::{cosine::angle_cos, linear_interpolation::lerp, matrix::Matrix, vector::Vector};

pub mod cosine;
pub mod linear_combination;
pub mod linear_interpolation;
pub mod matrix;
pub mod norm;
pub mod vector;

fn main() {
    // *
    println!("\nVector constructors\n");

    let vec = Vector::<f64>::new(4);
    println!("vec: {} (shape: {:#?})", vec, vec.shape());
    let vec = Vector::from(vec![1., 2., 3., 4.]);
    println!("vec: {} (shape: {:#?})", vec, vec.shape());
    let vec = Vector::from([1., 2., 3., 4.]);
    println!("vec: {} (shape: {:#?})", vec, vec.shape());
    let vec = Vector::from([1., 2., 3., 4.]);
    println!(
        "vec-to-mat: {} (shape: {:#?})",
        vec.reshape(),
        vec.reshape().shape()
    );

    // *
    println!("\nMatrix constructors\n");

    let matrix = Matrix::<f64>::new([4, 4]);
    println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());
    let matrix = Matrix::from(vec![1., 2., 3., 4., 5., 6.]);
    println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());
    let matrix = Matrix::from(vec![vec![1., 2., 3.], vec![4., 5., 6.]]);
    println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());
    let matrix = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());
    let matrix = Matrix::identity(4, 1.);
    println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());

    for column in matrix.iter_cols() {
        println!("[{}]", column);
    }

    // *
    println!("\nMatrix identity\n");

    let matrix_a = Matrix::identity(4, 1.);
    let matrix_b = Matrix::identity(4, 1.);
    let matrix_c = Matrix::map(&matrix_a, &matrix_b, |a, b| a + b);
    if let Ok(matrix) = matrix_c {
        println!("matrix: {} (shape: {:#?})", matrix, matrix.shape());
    } else {
        println!("failed to map matrix_a and matrix_b");
    }

    // *
    println!("\nLinear interpolation\n");

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

    // *
    println!("\nVector dot product\n");

    let u = Vector::from([0., 0.]);
    let v = Vector::from([1., 1.]);
    println!("{} (0.0)", u.dot(&v));
    let u = Vector::from([1., 1.]);
    let v = Vector::from([1., 1.]);
    println!("{} (2.0)", u.dot(&v));
    let u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    println!("{} (9.0)", u.dot(&v));

    // *
    println!("\nNorm\n");

    let u = Vector::from([0., 0., 0.]);
    println!(
        "{}, {}, {} (0.0, 0.0, 0.0)",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );
    let u = Vector::from([1., 2., 3.]);
    println!(
        "{}, {}, {} (6.0, 3.74165738, 3.0)",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );
    let u = Vector::from([-1., -2.]);
    println!(
        "{}, {}, {} (3.0, 2.236067977, 2.0)",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );

    // *
    println!("\nCosine\n");

    let u = Vector::from([1., 0.]);
    let v = Vector::from([1., 0.]);
    println!("{} (1.0)", angle_cos(&u, &v));
    let u = Vector::from([1., 0.]);
    let v = Vector::from([0., 1.]);
    println!("{} (0.0)", angle_cos(&u, &v));
    let u = Vector::from([-1., 1.]);
    let v = Vector::from([1., -1.]);
    println!("{} (-1.0)", angle_cos(&u, &v));
    let u = Vector::from([2., 1.]);
    let v = Vector::from([4., 2.]);
    println!("{} (1.0)", angle_cos(&u, &v));
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{} (0.97463185)", angle_cos(&u, &v));
}
