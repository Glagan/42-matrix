use crate::vector::Vector;

pub fn angle_cos(u: &Vector, v: &Vector) -> f64 {
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
