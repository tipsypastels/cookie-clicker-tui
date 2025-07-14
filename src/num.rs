use std::fmt;

macro_rules! big {
    (thousand) => {{ 1_000_u128 }};
    (million) => {{ 1_000_000_u128 }};
    (billion) => {{ 1_000_000_000_u128 }};
    (trillion) => {{ 1_000_000_000_000_u128 }};
    (quadrillion) => {{ 1_000_000_000_000_000_u128 }};
    (quintillion) => {{ 1_000_000_000_000_000_000_u128 }};
    (sextillion) => {{ 1_000_000_000_000_000_000_000_u128 }};
    (septillion) => {{ 1_000_000_000_000_000_000_000_000_u128 }};
    (octillion) => {{ 1_000_000_000_000_000_000_000_000_000_u128 }};
    ($m:ident f) => {{ big!($m) as f64 }};
    ($n:literal $m:ident) => {{ $n * big!($m) }};
    ($n:literal $m:ident f) => {{ $n * big!($m f) }};
}

pub(crate) use big;

#[derive(Debug, Copy, Clone)]
pub enum BigCountFmt {
    F64(f64),
    U128(u128),
}

impl fmt::Display for BigCountFmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (float, int) = match *self {
            Self::F64(f) => (Some(f), f.floor() as u128),
            Self::U128(i) => (None, i),
        };

        macro_rules! bases {
            ($($base:ident)*) => {
                $(if int >= big!($base) {
                    if let Some(float) = float {
                        let quot = float / big!($base f);
                        if quot == quot.floor() {
                            return write!(f, concat!("{} ", stringify!($base)), quot);
                        } else {
                            return write!(f, concat!("{:.2} ", stringify!($base)), quot);
                        }
                    } else {
                        let quot = int / big!($base);
                        return write!(f, concat!("{} ", stringify!($base)), quot);
                    }
                })*
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

        write!(f, "{int}")
    }
}

pub trait AsBigCountFmt {
    fn as_big_count_fmt(&self) -> BigCountFmt;
}

impl AsBigCountFmt for f64 {
    fn as_big_count_fmt(&self) -> BigCountFmt {
        BigCountFmt::F64(*self)
    }
}

impl AsBigCountFmt for u128 {
    fn as_big_count_fmt(&self) -> BigCountFmt {
        BigCountFmt::U128(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use BigCountFmt::*;

    macro_rules! assert_fmt {
        ($expr:expr, $str:literal) => {
            assert_eq!($expr.to_string(), $str)
        };
    }

    #[test]
    fn base() {
        assert_fmt!(F64(10.0), "10");

        assert_fmt!(U128(10), "10");
    }

    #[test]
    fn thousand() {
        assert_fmt!(F64(big!(1.0 thousand f)), "1 thousand");
        assert_fmt!(F64(big!(1.23 thousand f)), "1.230 thousand");
        assert_fmt!(F64(big!(29.596 thousand f)), "29.596 thousand");

        assert_fmt!(U128(big!(1 thousand)), "1 thousand");
        assert_fmt!(U128(big!(29 thousand)), "29 thousand");
    }

    #[test]
    fn million() {
        assert_fmt!(F64(big!(1.0 million f)), "1 million");
        assert_fmt!(F64(big!(1.23 million f)), "1.230 million");
        assert_fmt!(F64(big!(29.596 million f)), "29.596 million");

        assert_fmt!(U128(big!(1 million)), "1 million");
        assert_fmt!(U128(big!(29 million)), "29 million");
    }
}
