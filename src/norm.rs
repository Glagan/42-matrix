pub trait Norm {
    fn abs(self: Self) -> f64;
    fn pow(self: Self, power: f64) -> f64;
}

impl Norm for f64 {
    fn abs(self: Self) -> f64 {
        return f64::abs(self);
    }

    fn pow(self: Self, power: f64) -> f64 {
        return f64::powf(self, power);
    }
}
