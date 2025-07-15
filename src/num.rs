pub mod big {
    pub const THOUSAND: u128 = 1_000;
    pub const MILLION: u128 = 1_000_000;
    pub const BILLION: u128 = 1_000_000_000;
    pub const TRILLION: u128 = 1_000_000_000_000;
    pub const QUADRILLION: u128 = 1_000_000_000_000_000;
    pub const QUINTILLION: u128 = 1_000_000_000_000_000_000;
    pub const SEXTILLION: u128 = 1_000_000_000_000_000_000_000;
    pub const SEPTILLION: u128 = 1_000_000_000_000_000_000_000_000;
    pub const OCTILLION: u128 = 1_000_000_000_000_000_000_000_000_000;
}

pub mod bigf {
    pub const THOUSAND: f64 = super::big::THOUSAND as _;
    pub const MILLION: f64 = super::big::MILLION as _;
    pub const BILLION: f64 = super::big::BILLION as _;
    pub const TRILLION: f64 = super::big::TRILLION as _;
    pub const QUADRILLION: f64 = super::big::QUADRILLION as _;
    pub const QUINTILLION: f64 = super::big::QUINTILLION as _;
    pub const SEXTILLION: f64 = super::big::SEXTILLION as _;
    pub const SEPTILLION: f64 = super::big::SEPTILLION as _;
    pub const OCTILLION: f64 = super::big::OCTILLION as _;
}
