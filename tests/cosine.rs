use matrix::{cosine::angle_cos, vector::Vector};

#[test]
fn cosine_example_1() {
    let e1 = Vector::from([1., 0.]);
    let e2 = Vector::from([1., 0.]);

    assert_eq!(angle_cos(&e1, &e2), 1.);
}

#[test]
fn cosine_example_2() {
    let e1 = Vector::from([1., 0.]);
    let e2 = Vector::from([0., 1.]);

    assert_eq!(angle_cos(&e1, &e2), 0.);
}

#[test]
fn cosine_example_3() {
    let e1 = Vector::from([-1., 1.]);
    let e2 = Vector::from([1., -1.]);

    let cos = angle_cos(&e1, &e2);
    let expected = -1.;
    assert_eq!(cos as f32, expected);
}

#[test]
fn cosine_example_4() {
    let e1 = Vector::from([2., 1.]);
    let e2 = Vector::from([4., 2.]);

    let cos = angle_cos(&e1, &e2);
    let expected = 1.;
    assert_eq!(cos as f32, expected);
}

#[test]
fn cosine_example_5() {
    let e1 = Vector::from([1., 2., 3.]);
    let e2 = Vector::from([4., 5., 6.]);

    let cos = angle_cos(&e1, &e2);
    let expected = 0.974631846;
    assert_eq!(cos as f32, expected);
}

#[test]
fn cosine_invalid_empty() {
    let e1 = Vector::<f64>::from([]);
    let e2 = Vector::<f64>::from([]);

    assert_eq!(angle_cos(&e1, &e2), f64::default());
}

#[test]
fn cosine_invalid_example() {
    let e1 = Vector::from([0., 0., 3.]);
    let e2 = Vector::from([1., 1.]);

    assert_eq!(angle_cos(&e1, &e2), f64::default());
}
