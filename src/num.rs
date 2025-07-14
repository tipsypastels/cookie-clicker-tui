macro_rules! big {
    ($n:literal K) => {{ $n * 1_000 }};
    ($n:literal M) => {{ $n * 1_000_000 }};
    ($n:literal B) => {{ $n * 1_000_000_000 }};
    ($n:literal T) => {{ $n * 1_000_000_000_000 }};
}

pub(crate) use big;
