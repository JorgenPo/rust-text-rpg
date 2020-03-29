use termion::terminal_size;
use termion::{color, cursor, clear, style};

use std::io::{Error, Write, Stdout, StdoutLock};
use termion::raw::{IntoRawMode, RawTerminal};
use std::cmp::max;

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    std::io::stdout().write_fmt(args).expect("Failed to write");
}

#[macro_export]
macro_rules! render {
    ($($arg:tt)*) => (
        _print(format_args!($($arg)*));
        std::io::stdout().flush().unwrap();
    );
}

pub struct TermSize {
    pub width: u16,
    pub height: u16
}

impl Default for TermSize {
    fn default() -> Self {
        TermSize{
            width: 80,
            height: 120,
        }
    }
}

pub enum Coordinate {
    Absolute(u16),
    Centered,
    Percent(u8),
    FromBorder(u16), // Pixels from border
}

pub struct Position {
    pub x: Coordinate,
    pub y: Coordinate,
}

impl Position {
    pub fn from(x: u16, y: u16) -> Self {
        Position {
            x: Coordinate::Absolute(x),
            y: Coordinate::Absolute(y)
        }
    }
}

pub struct Render {
    pub term_size: TermSize,
    pub clear_color: Box<dyn color::Color>,
    pub hide_cursor: bool
}

pub trait Drawable {
    fn draw(&self) -> String;
    fn get_width(&self) -> u16;
    fn get_height(&self) -> u16;
    fn get_position(&self) -> &Position;
    fn set_position(&mut self, pos: Position);
}

impl Render {
    /// Constructs new renderer with default preferences
    pub fn new() -> Self {
        let mut term_size = match terminal_size() {
            Ok((width, height)) => TermSize{width, height},
            Err(err) => TermSize::default(),
        };

        if term_size.width == 0 || term_size.height == 0 {
            term_size = TermSize::default();
        }

        Render {
            term_size,
            clear_color: Box::new(color::Black),
            hide_cursor: true
        }
    }

    pub fn clear_screen(&mut self) {
        render!("{}{}{}",
               color::Bg(self.clear_color.as_ref()),
               clear::All,
               cursor::Goto(1, 1));

        if self.hide_cursor {
            render!("{}", cursor::Hide);
        }
    }

    fn get_middle_x<T: Drawable> (&self, drawable: &T) -> u16 {
        let half_width = drawable.get_width() / 2;
        let center_x = self.term_size.width / 2;

        center_x - half_width + 1
    }

    fn get_middle_y<T: Drawable> (&self, drawable: &T) -> u16 {
        let half_height = drawable.get_height() / 2;
        let center_y = self.term_size.height / 2;

        center_y - half_height + 1
    }

    pub fn set_cursor_position(&mut self, coord: (u16, u16)) {
        render!("{}", cursor::Goto(coord.0, coord.1));
    }

    pub fn set_pixel_color(&mut self, coord: (u16, u16), color: Box<dyn color::Color>) {
        print!("{}{} ", cursor::Goto(coord.0, coord.1), color::Bg(color.as_ref()));
    }

    pub fn draw<T: Drawable>(&mut self, drawable: &T) {
        let position = drawable.get_position();

        let x = match position.x {
            Coordinate::Absolute(x) => max(x, 1),
            Coordinate::Centered => self.get_middle_x(drawable),
            Coordinate::Percent(percent) => max(self.term_size.width,
                                                self.term_size.width * percent as u16 / 100),
            Coordinate::FromBorder(x) => max(1, self.term_size.width - x)
        };

        let y = match position.y {
            Coordinate::Absolute(y) => max(y, 1),
            Coordinate::Centered => self.get_middle_y(drawable),
            Coordinate::Percent(percent) => max(self.term_size.height,
                                                self.term_size.height * percent as u16 / 100),
            Coordinate::FromBorder(y) => max(1, self.term_size.height - y)
        };

        render!("{}{}", cursor::Goto(x, y), drawable.draw());
    }

    pub fn draw_raw(&mut self, string: &str) {
        render!("{}", string);
    }

    pub fn flash(&self) {
        std::io::stdout().flush().unwrap();
    }
}

impl Drop for Render {
    fn drop(&mut self) {
        print!("{}{}{}{}", clear::All, style::Reset, cursor::Show, cursor::Goto(1, 1));
    }
}
