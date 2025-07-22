use std::fmt;

pub fn pluralized<T>(n: usize, singular: T, plural: T) -> T {
    if n == 1 { singular } else { plural }
}

pub fn indefinite_article(s: &str) -> impl fmt::Display {
    struct IndefiniteArticle<'a> {
        article: &'static str,
        s: &'a str,
    }
    impl fmt::Display for IndefiniteArticle<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} {}", self.article, self.s)
        }
    }
    let article = match s.chars().next() {
        Some('a' | 'e' | 'i' | 'o' | 'u') => "an",
        _ => "a",
    };
    IndefiniteArticle { article, s }
}
