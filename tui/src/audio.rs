use anyhow::Result;
use soloud::{AudioExt, LoadExt, Soloud, Wav};

pub struct Audio {
    soloud: Soloud,
    golden_cookie_click: Wav,
}

impl Audio {
    pub fn new() -> Result<Self> {
        Ok(Self {
            soloud: Soloud::default()?,
            golden_cookie_click: {
                let mut wav = Wav::default();
                wav.load_mem(include_bytes!("../audio/golden_cookie_click.mp3"))?;
                wav
            },
        })
    }

    pub fn golden_cookie_click(&self) {
        self.soloud.play(&self.golden_cookie_click);
    }
}
