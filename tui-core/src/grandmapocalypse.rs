use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct Grandmapocalypse(Inner);

#[derive(Debug)]
enum Inner {
    Off,
    On { phase: GrandmapocalypsePhase },
}

impl Grandmapocalypse {
    pub(crate) fn new() -> Self {
        Self(Inner::Off)
    }

    fn from_repr(repr: Repr) -> Self {
        Self(match repr {
            Repr::Off => Inner::Off,
            Repr::On { phase } => Inner::On { phase },
        })
    }

    fn as_repr(&self) -> Repr {
        match &self.0 {
            Inner::Off => Repr::Off,
            Inner::On { phase } => Repr::On { phase: *phase },
        }
    }

    pub fn phase(&self) -> Option<GrandmapocalypsePhase> {
        match &self.0 {
            Inner::Off => None,
            Inner::On { phase, .. } => Some(*phase),
        }
    }

    pub(crate) fn set_phase(&mut self, phase: GrandmapocalypsePhase) {
        self.0 = Inner::On { phase };
    }
}
impl Serialize for Grandmapocalypse {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.as_repr().serialize(ser)
    }
}

impl<'de> Deserialize<'de> for Grandmapocalypse {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        Repr::deserialize(de).map(Self::from_repr)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GrandmapocalypsePhase {
    Awoken,
    Displeased,
    Angered,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "state")]
enum Repr {
    Off,
    On { phase: GrandmapocalypsePhase },
}
