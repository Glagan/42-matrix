use crate::vector::Vector;

pub fn cross_product(u: &Vector, v: &Vector) -> Vector {
    // * Validate
    let u_size = u.size();
    let v_size = v.size();

    if u_size != 3 || v_size != 3 {
        // return Err("Invalid vector length, expected 3".to_string());
        return Vector::new(0);
    }

    // * Simple formula

    Vector::from([
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ])
}
