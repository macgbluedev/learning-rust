use std::fmt::{self};

use super::{Label, Widget};

pub struct Button {
    label: Label,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Self {
            label: Label::new(label),
        }
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        // Add a little padding.
        self.label.width() + 8
    }

    fn draw_into(&self, buffer: &mut dyn fmt::Write) {
        let width = self.width();
        let mut label_buf = String::new();
        self.label.draw_into(&mut label_buf);

        writeln!(buffer, "+{:-<width$}+", "").unwrap();
        for line in label_buf.lines() {
            writeln!(buffer, "|{:^width$}|", line).unwrap();
        }
        writeln!(buffer, "+{:-<width$}+", "").unwrap();
    }
}