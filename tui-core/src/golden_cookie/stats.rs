use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Stats {
    pub click_count: usize,
    pub click_miss_count: usize,
    pub clicked_one_at_most_1s_after_spawn: bool,
    pub clicked_one_at_most_1s_before_despawn: bool,
}

impl Stats {
    pub fn add_click_count(&mut self) {
        self.click_count = self.click_count.saturating_add(1);
    }

    pub fn add_click_miss_count(&mut self) {
        self.click_miss_count = self.click_miss_count.saturating_add(1);
    }

    pub fn set_clicked_at_most_1s_after_spawn(&mut self) {
        self.clicked_one_at_most_1s_after_spawn = true;
    }

    pub fn set_clicked_at_most_1s_before_despawn(&mut self) {
        self.clicked_one_at_most_1s_before_despawn = true;
    }
}
