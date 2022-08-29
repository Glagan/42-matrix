use matrix::vector::Vector;

#[test]
fn norm_1() {
    assert_eq!(Vector::from([0., 0., 0.]).norm_1(), 0.);
    assert_eq!(Vector::from([1., 2., 3.]).norm_1(), 6.);
    assert_eq!(Vector::from([-1., -2.]).norm_1(), 3.);
}

#[test]
fn euclidean_norm() {
    assert_eq!(Vector::from([0., 0., 0.]).norm(), 0.);
    assert_eq!(Vector::from([1., 2., 3.]).norm(), 3.7416573867739413);
    assert_eq!(Vector::from([-1., -2.]).norm(), 2.23606797749979);
}

#[test]
fn supremum_norm() {
    assert_eq!(Vector::from([0., 0., 0.]).norm_inf(), 0.);
    assert_eq!(Vector::from([1., 2., 3.]).norm_inf(), 3.);
    assert_eq!(Vector::from([-1., -2.]).norm_inf(), 2.);
}
