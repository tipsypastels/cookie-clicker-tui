use crate::num::{big, bigf};
use std::fmt;

pub trait PrintNum {
    type With;

    fn print_num_with(self, with: Self::With) -> impl fmt::Display;
    fn print_num(self) -> impl fmt::Display
    where
        Self: PrintNum<With = ()> + Sized,
    {
        self.print_num_with(())
    }
}

impl PrintNum for f64 {
    type With = PrintFloat;

    fn print_num_with(self, with: Self::With) -> impl fmt::Display {
        #[derive(Copy, Clone)]
        struct Printer(f64, PrintFloat);

        impl fmt::Display for Printer {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let Self(n, mode) = self;
            }
        }

        Printer(self, with)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PrintFloat {
    Floor,
    Precision(usize),
}

macro_rules! all_the_bases {
    ($macro:ident) => {
        $macro!(OCTILLION, "octillion");
        $macro!(SEPTILLION, "septillion");
        $macro!(SEXTILLION, "sextillion");
        $macro!(QUINTILLION, "quintillion");
        $macro!(QUADRILLION, "quadrillion");
        $macro!(TRILLION, "trillion");
        $macro!(BILLION, "billion");
        $macro!(MILLION, "million");
        $macro!(THOUSAND, "thousand");
    };
}

use all_the_bases;

// impl PrintNum for f64 {
//     type With = PrintFloat;

//     fn print_num_with(self, mode: PrintFloat) -> impl fmt::Display {
//         Impl::F64(self, mode)
//     }
// }

// impl PrintNum for u128 {
//     type With = ();

//     fn print_num_with(self, (): ()) -> impl fmt::Display {
//         Impl::U128(self)
//     }
// }

// #[derive(Debug, Copy, Clone)]
// pub enum PrintFloat {
//     Floor,
//     Precision(usize),
// }

// #[derive(Copy, Clone)]
// enum Impl {
//     F64(f64, PrintFloat),
//     U128(u128),
// }

// impl fmt::Display for Impl {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         macro_rules! all_the_bases {
//             ($macro:ident) => {
//                 $macro!(OCTILLION, "octillion");
//                 $macro!(SEPTILLION, "septillion");
//                 $macro!(SEXTILLION, "sextillion");
//                 $macro!(QUINTILLION, "quintillion");
//                 $macro!(QUADRILLION, "quadrillion");
//                 $macro!(TRILLION, "trillion");
//                 $macro!(BILLION, "billion");
//                 $macro!(MILLION, "million");
//                 $macro!(THOUSAND, "thousand");
//             };
//         }

//         match *self {
//             Self::F64(n, PrintFloat::Floor) => Self::U128(n.floor() as _).fmt(f),
//             Self::F64(n, PrintFloat::Precision(p)) => {
//                 macro_rules! base {
//                     ($base:ident, $label:literal) => {{
//                         use bigf::*;

//                         if n >= $base {
//                             let quot = n / $base;
//                             return write!(f, concat!("{:.1$} ", $label), quot, p);
//                         }
//                     }};
//                 }

//                 all_the_bases!(base);
//                 write!(f, "{n:.p$}")
//             }
//             Self::U128(n) => {
//                 macro_rules! base {
//                     ($base:ident, $label:literal) => {{
//                         use big::*;

//                         if n >= $base {
//                             let quot = n / $base;
//                             return write!(f, concat!("{} ", $label), quot);
//                         }
//                     }};
//                 }

//                 all_the_bases!(base);
//                 write!(f, "{n}")
//             }
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use PrintFloat::*;

//     macro_rules! assert_fmt {
//         ($expr:expr, $str:literal) => {
//             assert_eq!($expr.to_string(), $str)
//         };
//     }

//     #[test]
//     fn base() {
//         assert_fmt!(10.print_num(), "10");
//         assert_fmt!(10.0.print_num_with(Floor), "10");
//         assert_fmt!(10.0.print_num_with(Precision(2)), "10.00");
//     }

//     #[test]
//     fn thousand() {
//         assert_fmt!(1000.print_num(), "1 thousand");
//         assert_fmt!(1000.0.print_num_with(Floor), "1 thousand");
//         assert_fmt!(1000.0.print_num_with(Precision(2)), "1.00 thousand");
//     }
// }
