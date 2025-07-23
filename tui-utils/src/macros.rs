#[macro_export]
macro_rules! enum_map {
    (
        $(#[$attr:meta])*
        $vis:vis struct $ident:ident of $enum:ident {
            $($(#[$fattr:meta])* $field:ident: $variant:ident),*$(,)?
        }
    ) => {
        $(#[$attr])*
        $vis struct $ident<T> {
            $($(#[$fattr])* $field: T),*
        }

        impl<T> $ident<T> {
            $vis fn new(f: impl Fn($enum) -> T) -> Self {
                Self {
                    $($field: f($enum::$variant)),*
                }
            }

            $vis fn get(&self, variant: $enum) -> &T {
                match variant {
                    $($enum::$variant => &self.$field),*
                }
            }

            $vis fn get_mut(&mut self, variant: $enum) -> &mut T {
                match variant {
                    $($enum::$variant => &mut self.$field),*
                }
            }
        }
    };
}
