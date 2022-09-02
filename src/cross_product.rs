use crate::{norm::Norm, vector::Vector};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign},
};

pub fn cross_product<
    K: Default
        + Clone
        + Copy
        + Debug
        + PartialEq
        + AddAssign
        + SubAssign
        + MulAssign
        + Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + Div<Output = K>
        + Mul<f64, Output = K>
        + Div<f64, Output = f64>
        + Norm,
>(
    u: &Vector<K>,
    v: &Vector<K>,
) -> Vector<K> {
    // * Validate
    let u_size = u.size();
    let v_size = v.size();

    if u_size != 3 || v_size != 3 {
        return Vector::new(0);
        // Err("Invalid vector length, expected 3".to_string());
    }

    Vector::from([
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ])
}
