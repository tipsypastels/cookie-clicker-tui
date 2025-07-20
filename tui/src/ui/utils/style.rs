use ratatui::style::{Color, Modifier, Style};

#[allow(unused)]
pub trait StyleExt {
    fn fg_if(self, cond: bool, color: Color) -> Self;
    fn patch_if(self, cond: bool, other: impl Into<Style>) -> Self;
    fn add_modifier_if(self, cond: bool, modifier: Modifier) -> Self;
}

impl StyleExt for Style {
    fn fg_if(self, cond: bool, color: Color) -> Self {
        if cond { self.fg(color) } else { self }
    }

    fn patch_if(self, cond: bool, other: impl Into<Style>) -> Self {
        if cond { self.patch(other) } else { self }
    }

    fn add_modifier_if(self, cond: bool, modifier: Modifier) -> Self {
        if cond {
            self.add_modifier(modifier)
        } else {
            self
        }
    }
}
