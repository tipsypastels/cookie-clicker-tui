use super::{building::Buildings, cookies::Cookies};

#[derive(Debug)]
pub struct State {
    pub buildings: Buildings,
    pub cookies: Cookies,
}

impl State {
    pub fn new() -> Self {
        Self {
            buildings: Buildings::new(),
            cookies: Cookies::new(),
        }
    }
}
