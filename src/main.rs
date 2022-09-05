use matrix::matrix::Matrix;

fn main() {
    let projection = Matrix::projection(90., 1., 1., 50.);
    for row in 0..4 {
        println!(
            "{:.1}, {:.1}, {:.1}, {:.1}",
            projection[row][0], projection[row][1], projection[row][2], projection[row][3]
        );
    }
}
