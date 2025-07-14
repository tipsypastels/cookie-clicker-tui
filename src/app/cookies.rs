use crate::{event::FPS, num::big};
use std::{cmp::Ordering, fmt, ops::RangeBounds};

#[derive(Debug, Copy, Clone)]
pub struct Cookies(f64);

impl Cookies {
    pub fn new() -> Self {
        Self(0.0)
    }

    pub fn tick(&mut self, cps: f64) {
        self.0 += cps / FPS;
    }

    pub fn add(&mut self, cookies: u32) {
        self.0 += cookies as f64;
    }

    pub fn sub(&mut self, cookies: u32) {
        self.0 -= cookies as f64;
    }

    pub fn value(self) -> u128 {
        self.0.floor() as _
    }

    pub fn value_f64(self) -> f64 {
        self.0
    }

    pub fn in_range(self, range: &impl RangeBounds<u128>) -> bool {
        range.contains(&self.value())
    }
}

impl PartialEq<u128> for Cookies {
    fn eq(&self, other: &u128) -> bool {
        self.value().eq(other)
    }
}

impl PartialOrd<u128> for Cookies {
    fn partial_cmp(&self, other: &u128) -> Option<Ordering> {
        self.value().partial_cmp(other)
    }
}

impl fmt::Display for Cookies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nf = self.0;
        let ni = self.value();

        macro_rules! bases {
            ($($base:ident)*) => {{
                $(
                    let base: u128 = big!($base);
                    if ni >= base {
                        let nfd = nf / base as f64;

                        if nfd == nfd.floor() {
                            return write!(f, concat!("{} ", stringify!($base)), nfd);
                        } else {
                            return write!(f, concat!("{:.3} ", stringify!($base)), nfd);
                        }
                    }
                )*
            }
            };
        }

        bases! {
            octillion
            septillion
            sextillion
            quintillion
            quadrillion
            trillion
            billion
            million
            thousand
        }

        write!(f, "{ni}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! big_cookies {
        ($n:literal $m:ident) => {{
            let base: u128 = big!($m);
            Cookies($n * base as f64)
        }};
    }

    macro_rules! assert_fmt {
        ($expr:expr, $str:literal) => {
            assert_eq!($expr.to_string(), $str)
        };
    }

    #[test]
    fn base() {
        assert_fmt!(Cookies(10.0), "10");
    }

    #[test]
    fn thousand() {
        assert_fmt!(big_cookies!(1.0 thousand), "1 thousand");
        assert_fmt!(big_cookies!(1.23 thousand), "1.230 thousand");
        assert_fmt!(big_cookies!(29.596 thousand), "29.596 thousand");
    }

    #[test]
    fn million() {
        assert_fmt!(big_cookies!(1.0 million), "1 million");
        assert_fmt!(big_cookies!(1.23 million), "1.230 million");
        assert_fmt!(big_cookies!(29.596 million), "29.596 million");
    }
}
