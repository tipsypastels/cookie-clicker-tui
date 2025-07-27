pub struct ThousandFingers {
    pub non_cursor_buildings_count: u16,
    pub mult: f64,
}

impl ThousandFingers {
    pub fn calc(self) -> f64 {
        0.1 * self.non_cursor_buildings_count as f64 * self.mult
    }
}
