use enum_assoc::Assoc;
use enum_fun::Variants;

#[derive(Assoc, Variants, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[func(pub const fn char(self) -> char)]
#[func(pub const fn from_char(ch: char) -> Option<Self>)]
pub enum InputChar {
    #[assoc(char = '1', from_char = '1')]
    One,
    #[assoc(char = '2', from_char = '2')]
    Two,
    #[assoc(char = '3', from_char = '3')]
    Three,
    #[assoc(char = '4', from_char = '4')]
    Four,
    #[assoc(char = '5', from_char = '5')]
    Five,
    #[assoc(char = '6', from_char = '6')]
    Six,
    #[assoc(char = '7', from_char = '7')]
    Seven,
    #[assoc(char = '8', from_char = '8')]
    Eight,
    #[assoc(char = '9', from_char = '9')]
    Nine,
}
