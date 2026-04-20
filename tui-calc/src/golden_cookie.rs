pub fn lucky_gain(cps: f64, cookies: f64) -> f64 {
    f64::min(cps * 900.0 + 13.0, cookies * 0.15 + 13.0)
}
