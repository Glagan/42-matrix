use matrix::{cross_product::cross_product, vector::Vector};

#[test]
fn cross_product_example_1() {
    let e1 = Vector::from([0., 0., 1.]);
    let e2 = Vector::from([1., 0., 0.]);

    let result = cross_product(&e1, &e2);
    let expected = &vec![0., 1., 0.];
    assert_eq!(result.all(), expected);
}

#[test]
fn cross_product_example_2() {
    let e1 = Vector::from([1., 2., 3.]);
    let e2 = Vector::from([4., 5., 6.]);

    let result = cross_product(&e1, &e2);
    let expected = &vec![-3., 6., -3.];
    assert_eq!(result.all(), expected);
}

#[test]
fn cross_product_example_3() {
    let e1 = Vector::from([4., 2., -3.]);
    let e2 = Vector::from([-2., -5., 16.]);

    let result = cross_product(&e1, &e2);
    let expected = &vec![17., -58., -16.];
    assert_eq!(result.all(), expected);
}

#[test]
fn cross_product_invalid_empty() {
    let e1 = Vector::<f64>::from([]);
    let e2 = Vector::<f64>::from([]);

    let result = cross_product(&e1, &e2);
    assert_eq!(result.size(), 0);
}

#[test]
fn cross_product_invalid_example() {
    let e1 = Vector::from([0., 0., 3.]);
    let e2 = Vector::from([1., 1.]);

    let result = cross_product(&e1, &e2);
    assert_eq!(result.size(), 0);
}
