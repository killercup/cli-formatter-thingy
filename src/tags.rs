use termcolor::{Color as CliColor, ColorSpec};

use super::Error;

pub struct Bold;

impl Bold {
    pub fn new() -> Bold {
        Bold
    }

    pub fn update(&self, color_spec: &mut ColorSpec) -> Result<(), Error> {
        color_spec.set_bold(true);
        Ok(())
    }
}

#[derive(Default)]
pub struct Color {
    fg: Option<CliColor>,
    bg: Option<CliColor>,
}

impl Color {
    pub fn new() -> Color {
        Color::default()
    }

    pub fn set(&mut self, field: &str, value: CliColor) {
        match field {
            "fg" => self.fg = Some(value),
            "bg" => self.bg = Some(value),
            _ => panic!("Cannot set value for field `{}`", field),
        }
    }

    pub fn update(&self, color_spec: &mut ColorSpec) -> Result<(), Error> {
        if let Some(ref _fg) = self.fg {
            color_spec.set_fg(self.fg.clone());
        }

        if let Some(ref _bg) = self.bg {
            color_spec.set_bg(self.bg.clone());
        }

        Ok(())
    }
}