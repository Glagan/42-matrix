use matrix::linear_combination::linear_combination;
use matrix::vector::Vector;

#[test]
fn linear_combination_example_1() {
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);

    let result = linear_combination(&[e1, e2, e3], &[10., -2., 0.5]);
    match result {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Failed to calculate linear combination"),
    }

    let expected = &vec![10., -2., 0.5];
    assert_eq!(result.unwrap().all(), expected);
}

#[test]
fn linear_combination_example_2() {
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);

    let result = linear_combination(&[v1, v2], &[10., -2.]);
    match result {
        Ok(_) => assert!(true),
        Err(ref err) => assert!(false, "Failed to calculate linear combination: {:#?}", err),
    }

    let expected = &vec![10., 0., 230.];
    assert_eq!(result.unwrap().all(), expected);
}

#[test]
fn invalid_linear_combination_vectors() {
    let v1 = Vector::from([1., 2., 3., 4.]);
    let v2 = Vector::from([0., 10., -100.]);

    let result = linear_combination(&[v1, v2], &[42., 0.]);
    match result {
        Ok(_) => assert!(false, "Failed to fail to calculate linear combination"),
        Err(_) => assert!(true),
    }
}

#[test]
fn invalid_linear_combination_coeffs() {
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);

    let result = linear_combination(&[v1, v2], &[42., 0., 42.]);
    match result {
        Ok(_) => assert!(false, "Failed to fail to calculate linear combination"),
        Err(_) => assert!(true),
    }
}
