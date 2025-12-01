use std::cmp::max;
use std::fmt::{self, Write};

/// Trait that all widgets must implement.
pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

/* ---------- Label ---------- */

pub struct Label {
    label: String,
}

impl Label {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_owned(),
        }
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        self.label
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0)
    }

    fn draw_into(&self, buffer: &mut dyn fmt::Write) {
        writeln!(buffer, "{}", self.label).unwrap();
    }
}

/* ---------- Button ---------- */

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

/* ---------- Window ---------- */

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    fn inner_width(&self) -> usize {
        max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        // 4 chars for the border
        self.inner_width() + 4
    }

    fn draw_into(&self, buffer: &mut dyn fmt::Write) {
        let mut inner = String::new();
        for widget in &self.widgets {
            widget.draw_into(&mut inner);
        }

        let inner_width = self.inner_width();

        writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
        writeln!(buffer, "| {:^inner_width$} |", self.title).unwrap();
        writeln!(buffer, "+={:=<inner_width$}=+", "").unwrap();

        for line in inner.lines() {
            writeln!(buffer, "| {:<inner_width$} |", line).unwrap();
        }

        writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
    }
}

/* ---------- Main ---------- */

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new(
        "This is a small text GUI demo.",
    )));
    window.add_widget(Box::new(Button::new("Click me!")));
    window.draw();
}
