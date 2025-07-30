pub fn pop_cookies(eaten: f64, shiny: bool) -> f64 {
    eaten * if shiny { 3.3 } else { 1.1 }
}
