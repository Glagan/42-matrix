use matrix::vector::Vector;

#[test]
fn dot_product_example_1() {
    let e1 = Vector::from([0., 0.]);
    let e2 = Vector::from([1., 1.]);

    assert_eq!(e1.dot(e2), 0.);
}

#[test]
fn dot_product_example_2() {
    let e1 = Vector::from([1., 1.]);
    let e2 = Vector::from([1., 1.]);

    assert_eq!(e1.dot(e2), 2.);
}

#[test]
fn dot_product_example_3() {
    let e1 = Vector::from([-1., 6.]);
    let e2 = Vector::from([3., 2.]);

    assert_eq!(e1.dot(e2), 9.);
}

#[test]
fn dot_product_invalid_example() {
    let e1 = Vector::from([0., 0., 3.]);
    let e2 = Vector::from([1., 1.]);

    assert_eq!(e1.dot(e2), f64::default());
}
