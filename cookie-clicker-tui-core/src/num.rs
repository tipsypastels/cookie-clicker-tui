pub const THOUSAND: f64 = 1e3;
pub const MILLION: f64 = 1e6;
pub const BILLION: f64 = 1e9;
pub const TRILLION: f64 = 1e12;
pub const QUADRILLION: f64 = 1e15;
pub const QUINTILLION: f64 = 1e18;
pub const SEXTILLION: f64 = 1e21;
pub const SEPTILLION: f64 = 1e24;
pub const OCTILLION: f64 = 1e27;
pub const NONILLION: f64 = 1e30;
pub const DECILLION: f64 = 1e33;
pub const UNDECILLION: f64 = 1e36;
pub const DUODECILLION: f64 = 1e39;
pub const TREDECILLION: f64 = 1e42;
pub const QUATTORDECILLION: f64 = 1e45;
pub const QUINDECILLION: f64 = 1e48;
pub const SEXDECILLION: f64 = 1e51;
pub const SEPTENDECILLION: f64 = 1e54;
pub const OCTODECILLION: f64 = 1e57;
pub const NOVEMDECILLION: f64 = 1e60;
pub const VIGINTILLION: f64 = 1e63;

pub trait ApproxEq<Rhs: ?Sized = Self> {
    fn approx_eq(&self, other: &Rhs) -> bool;
}

// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
impl ApproxEq for f64 {
    fn approx_eq(&self, other: &Self) -> bool {
        let diff = (self - other).abs();
        let a = self.abs();
        let b = other.abs();
        let largest = f64::max(a, b);
        diff <= largest * f64::EPSILON
    }
}

#[cfg(test)]
impl ApproxEq for [f64] {
    fn approx_eq(&self, other: &Self) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .enumerate()
                .all(|(i, &a)| a.approx_eq(&other[i]))
    }
}

#[cfg(test)]
impl<const N: usize> ApproxEq for [f64; N] {
    fn approx_eq(&self, other: &Self) -> bool {
        (self as &[f64]).approx_eq(other)
    }
}

macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        assert!(
            $crate::num::ApproxEq::approx_eq(a, b),
            "expected {a:?} and {b:?} to be approx equal",
        );
    }};
}

pub(crate) use assert_approx_eq;
