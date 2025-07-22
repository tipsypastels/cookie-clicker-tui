use crate::State;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Cost {
    Cookies(f64),
}

impl Cost {
    pub(crate) fn affordable(self, state: &State) -> bool {
        match self {
            Self::Cookies(c) => c <= state.cookies.current(),
        }
    }

    pub(crate) fn total_cmp(a: Self, b: Self) -> Ordering {
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        enum CostKind {
            Cookies,
        }
        impl From<Cost> for CostKind {
            fn from(cost: Cost) -> Self {
                match cost {
                    Cost::Cookies(_) => Self::Cookies,
                }
            }
        }
        CostKind::from(a)
            .cmp(&CostKind::from(b))
            .then_with(|| match (a, b) {
                (Self::Cookies(a), Self::Cookies(b)) => f64::total_cmp(&a, &b),
                #[allow(unreachable_patterns)] // for future expansion
                _ => unreachable!(),
            })
    }
}
