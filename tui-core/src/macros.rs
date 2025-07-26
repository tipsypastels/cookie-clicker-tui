macro_rules! impl_serde_from_state {
    ($ty:ty as $state_field:ident: $state_ty:ty) => {
        impl serde::Serialize for $ty {
            fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
                self.$state_field.serialize(ser)
            }
        }

        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
                <$state_ty>::deserialize(de).map(Self::from_state)
            }
        }
    };
}

pub(crate) use impl_serde_from_state;
