use crate::{
    cosine::angle_cos, cross_product::cross_product, linear_interpolation::lerp, matrix::Matrix,
    vector::Vector,
};

pub mod cosine;
pub mod cross_product;
pub mod linear_combination;
pub mod linear_interpolation;
pub mod matrix;
pub mod norm;
pub mod vector;

fn main() {
    // *
    println!("\nVector constructors\n");

    let vec = Vector::new(4);
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

    let matrix = Matrix::new([4, 4]);
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
        "{}\n([2.6, 1.3])",
        lerp(&Vector::from([2., 1.]), &Vector::from([4., 2.]), 0.3)
    );
    println!(
        "{}\n([[11.0, 5.5], [16.5, 22.0]])",
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

    // *
    println!("\nCross product\n");

    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    println!("{}\n([0., 1., 0.])", cross_product(&u, &v));
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}\n([-3., 6., -3.])", cross_product(&u, &v));
    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    println!("{}\n([17., -58., -16.])", cross_product(&u, &v));

    // *
    println!("\nMatrix multiplication\n");

    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Vector::from([4., 2.]);
    println!("{}\n([4., 2.])", u.mul_vec(&v));
    let u = Matrix::from([[2., 0.], [0., 2.]]);
    let v = Vector::from([4., 2.]);
    println!("{}\n([8., 4.])", u.mul_vec(&v));
    let u = Matrix::from([[2., -2.], [-2., 2.]]);
    let v = Vector::from([4., 2.]);
    println!("{}\n([4., -4.])", u.mul_vec(&v));
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[1., 0.], [0., 1.]]);
    println!("{}\n([[1., 0.], [0., 1.]])", u.mul_mat(&v));
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!("{}\n([[2., 1.], [4., 2.]])", u.mul_mat(&v));
    let u = Matrix::from([[3., -5.], [6., 8.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!("{}\n([[-14., -7.], [44., 22.]])", u.mul_mat(&v));

    // *
    println!("\nMatrix trace\n");

    let u = Matrix::from([[1., 0.], [0., 1.]]);
    println!("{} (2.)", u.trace());
    let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    println!("{} (9.)", u.trace());
    let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    println!("{} (-21.)", u.trace());

    // *
    println!("\nMatrix transpose\n");

    let u = Matrix::from([[1., 2.], [3., 1.]]);
    println!("{}\n([[1., 3.], [2., 1.]])", u.transpose());
    let u = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    println!("{}\n([[1.0, 4.0], [2.0, 5.0], [3.0, 6.0]])", u.transpose());
    let u = Matrix::from([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
    println!(
        "{}\n([[1.0, 4.0, 7.0], [2.0, 5.0, 8.0], [3.0, 6.0, 9.0]])",
        u.transpose()
    );

    // *
    println!("\nMatrix reduced row echelon form\n");

    let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    println!(
        "{}\n([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])",
        u.row_echelon()
    );
    let u = Matrix::from([[1., 2.], [3., 4.]]);
    println!("{}\n([[1., 0.], [0., 1.]])", u.row_echelon());
    let u = Matrix::from([[1., 2.], [2., 4.]]);
    println!("{}\n([[1., 2.], [0., 0.]])", u.row_echelon());
    let u = Matrix::from([
        [8., 5., -2., 4., 28.],
        [4., 2.5, 20., 4., -4.],
        [8., 5., 1., 4., 17.],
    ]);
    println!("{}\n([[1., 0.625, 0., 0., -12.1666667], [0., 0., 1., 0., -3.6666667], [0., 0., 0., 1., 29.5]])", u.row_echelon());

    // *
    println!("\nMatrix determinant\n");

    let u = Matrix::from([[1., -1.], [-1., 1.]]);
    println!("{} (0.0)", u.determinant());
    let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    println!("{} (8.0)", u.determinant());
    let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    println!("{} (-174.0)", u.determinant());
    let u = Matrix::from([
        [8., 5., -2., 4.],
        [4., 2.5, 20., 4.],
        [8., 5., 1., 4.],
        [28., -4., 17., 1.],
    ]);
    println!("{} (1032.0)", u.determinant());

    // *
    println!("\nMatrix inverse\n");

    let u = Matrix::from([[1., 2.], [3., 4.]]);
    println!("{} ([[-2., 1.], [1.5, -0.5]])", u.inverse().unwrap(),);
    let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    println!(
        "{}\n([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])",
        u.inverse().unwrap()
    );
    let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    println!(
        "{}\n([[0.5, 0.0, 0.0], [0.0, 0.5, 0.0], [0.0, 0.0, 0.5]])",
        u.inverse().unwrap()
    );
    let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    println!(
        "{}\n([[0.649425287, 0.097701149, -0.655172414], [-0.781609195, -0.126436782, 0.965517241], [0.143678161, 0.074712644, -0.206896552]])",
        u.inverse().unwrap()
    );

    // *
    println!("\nMatrix rank\n");

    let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    println!("{} (3)", u.rank());
    let u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
    println!("{} (2)", u.rank());
    let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
    println!("{} (3)", u.rank());
}
