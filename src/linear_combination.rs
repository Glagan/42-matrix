use crate::vector::Vector;

pub fn linear_combination(vectors: &[Vector], coeffs: &[f64]) -> Result<Vector, String> {
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
        let mut value = 0.;
        for (index, vector) in vectors.iter().enumerate() {
            value += vector[column] * coeffs[index];
        }
        new_vector[column] = value;
    }

    Ok(new_vector)
}
