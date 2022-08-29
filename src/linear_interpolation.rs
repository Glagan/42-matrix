pub trait Lerp {
    // Linear interpolation precise method
    // @see https://en.wikipedia.org/wiki/Linear_interpolation#Programming%20language%20support
    fn lerp(a: &Self, b: &Self, t: f64) -> Self;
}

impl Lerp for f64 {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        a * (1. - t) + b * t
    }
}

pub fn lerp<K: Lerp>(a: &K, b: &K, t: f64) -> K {
    K::lerp(a, b, t)
}
