use crate::frames::FPS;

#[derive(Debug)]
pub struct Refresh {
    cur: f64,
    max: f64,
}

impl Refresh {
    pub fn new(max_secs: f64) -> Self {
        Self {
            cur: 0.0,
            max: max_secs * FPS,
        }
    }

    pub fn reset(&mut self) {
        self.cur = 0.0;
    }

    pub fn finish(&mut self) -> bool {
        let next = self.cur + 1.0;
        if next >= self.max {
            self.cur = 0.0;
            true
        } else {
            self.cur = next;
            false
        }
    }

    pub fn modify(&mut self, f: impl FnOnce(&mut f64)) {
        f(&mut self.max);
    }
}
