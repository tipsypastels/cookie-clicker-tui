use cookie_clicker_tui_core::Changeset;

#[derive(Default, Debug)]
pub struct AppChangesetState {
    latest_significant: Option<Changeset>,
    time_ago: usize,
}

impl AppChangesetState {
    pub(super) fn tick(&mut self, changeset: Changeset) {
        if changeset != Changeset::default() {
            self.latest_significant = Some(changeset);
            self.time_ago = 0;
        } else {
            self.time_ago = self.time_ago.wrapping_add(1);
        }
    }
}
