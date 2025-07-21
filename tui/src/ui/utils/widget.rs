#![allow(unused)]

use ratatui::prelude::*;

pub trait WidgetExt: Widget + Sized {
    fn render_stateless(self, area: Rect, buf: &mut Buffer) {
        Self::render(self, area, buf);
    }
}

pub trait StatefulWidgetExt: StatefulWidget + Sized {
    fn render_stateful(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Self::render(self, area, buf, state);
    }
}

pub trait MaybeStatefulWidgetExt: Widget + StatefulWidget + Sized {
    fn render_maybe_stateful(self, area: Rect, buf: &mut Buffer, state: Option<&mut Self::State>) {
        if let Some(state) = state {
            self.render_stateful(area, buf, state);
        } else {
            self.render_stateless(area, buf);
        }
    }
}

pub trait StatefulOrDefaultStateWidget: StatefulWidget + Sized
where
    Self::State: Default,
{
    fn render_stateful_or_default_state(
        self,
        area: Rect,
        buf: &mut Buffer,
        state: Option<&mut Self::State>,
    ) {
        if let Some(state) = state {
            self.render_stateful(area, buf, state);
        } else {
            let mut state = Self::State::default();
            self.render_stateful(area, buf, &mut state);
        }
    }
}

impl<W: Widget> WidgetExt for W {}
impl<W: StatefulWidget> StatefulWidgetExt for W {}
impl<W: Widget + StatefulWidget> MaybeStatefulWidgetExt for W {}
impl<W: StatefulWidget<State: Default>> StatefulOrDefaultStateWidget for W {}
