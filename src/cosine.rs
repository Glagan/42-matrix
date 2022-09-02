use crate::{norm::Norm, vector::Vector};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign},
};

pub fn angle_cos<
    K: Default
        + Clone
        + Copy
        + Debug
        + AddAssign
        + SubAssign
        + MulAssign
        + Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + Mul<f64, Output = K>
        + Div<f64, Output = f64>
        + Norm,
>(
    u: &Vector<K>,
    v: &Vector<K>,
) -> f64 {
    // * Validate
    let u_size = u.size();
    let v_size = v.size();

    if u_size == 0 || v_size == 0 {
        return 0.; // Err("Empty vectors to transform".to_string());
    }
    if u_size != v_size {
        return 0.;
        /* Err(format!(
            "Invalid vector length, got {} expected {}",
            v_size, u_size
        )); */
    }

    // * Calculate the cosine angle between the vectors
    // ? (A dot B) / (||A|| ||B||)
    u.dot(v) / (u.norm() * v.norm())
}
