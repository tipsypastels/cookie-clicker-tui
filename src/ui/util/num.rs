use crate::num;
use std::fmt;

pub trait PrintFloat {
    fn print_float(
        self,
        precision_below_base: usize,
        precision_above_base: usize,
    ) -> impl fmt::Display;
}

impl PrintFloat for f64 {
    fn print_float(
        self,
        precision_below_base: usize,
        precision_above_base: usize,
    ) -> impl fmt::Display {
        Display(self, precision_below_base, precision_above_base)
    }
}

#[derive(Copy, Clone)]
struct Display(f64, usize, usize);

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(n, precision_below_base, precision_above_base) = *self;

        macro_rules! all_the_bases {
            ($macro:ident) => {
                $macro!(OCTILLION, " Oc");
                $macro!(SEPTILLION, " Sp");
                $macro!(SEXTILLION, " Sx");
                $macro!(QUINTILLION, " Qi");
                $macro!(QUADRILLION, " Qa");
                $macro!(TRILLION, " T");
                $macro!(BILLION, " B");
                $macro!(MILLION, " M");
                $macro!(THOUSAND, "k");
            };
        }

        macro_rules! base {
            ($base:ident, $label:literal) => {
                if n >= num::$base {
                    let quot = n / num::$base;

                    if quot == quot.floor() {
                        return write!(f, concat!("{:.0}", $label), quot);
                    }

                    return write!(f, concat!("{:.1$}", $label), quot, precision_above_base);
                }
            };
        }

        all_the_bases!(base);

        if n == n.floor() {
            write!(f, "{n:.0}")
        } else {
            write!(f, "{n:.0$}", precision_below_base)
        }
    }
}
