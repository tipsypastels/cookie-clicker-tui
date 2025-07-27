use crate::thousand_fingers::ThousandFingers;

pub enum Cpc {
    Basic,
    ThousandFingers(ThousandFingers),
}

impl Cpc {
    pub fn calc(self) -> f64 {
        match self {
            Self::Basic => 1.0,
            Self::ThousandFingers(tf) => 1.0 + tf.calc(),
        }
    }
}
