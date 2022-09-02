use crate::{norm::Norm, vector::Vector};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign},
};

pub fn linear_combination<
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
        + Norm,
>(
    vectors: &[Vector<K>],
    coeffs: &[K],
) -> Result<Vector<K>, String> {
    // * Validate

    if vectors.is_empty() {
        return Err("Empty vectors to transform".to_string());
    }
    if coeffs.is_empty() {
        return Err("Empty coefficients to apply".to_string());
    }

    let size = vectors[0].size();
    for vector in vectors {
        if vector.size() != size {
            return Err(format!(
                "Invalid vector length, got {} expected {}",
                vector.size(),
                size
            ));
        }
    }

    if coeffs.len() != vectors.len() {
        return Err(format!(
            "Invalid coefficents length, got {} expected {}",
            coeffs.len(),
            vectors.len()
        ));
    }

    // * Calculate the linear combination on a new vector

    let mut new_vector = Vector::new(size);
    for column in 0..size {
        let mut value = K::default();
        for (index, vector) in vectors.iter().enumerate() {
            value += vector[column] * coeffs[index];
        }
        new_vector[column] = value;
    }

    Ok(new_vector)
}
