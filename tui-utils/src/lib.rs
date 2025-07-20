pub mod countdown;
pub mod num;

pub fn pluralized<T>(n: usize, singular: T, plural: T) -> T {
    if n == 1 { singular } else { plural }
}
