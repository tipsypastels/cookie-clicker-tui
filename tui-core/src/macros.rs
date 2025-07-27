macro_rules! deserialize_via_state {
    ($ty:ty => $state_ty:ty as $from_state:path) => {
        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
                <$state_ty>::deserialize(de).map($from_state)
            }
        }
    };
}

macro_rules! serialize_via_state {
    ($ty:ty => $state_ty:ty as |$self_ident:ident| $to_state:expr) => {
        impl serde::Serialize for $ty {
            fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
                let $self_ident = self;
                $to_state.serialize(ser)
            }
        }
    };
}

pub(crate) use deserialize_via_state;
pub(crate) use serialize_via_state;
