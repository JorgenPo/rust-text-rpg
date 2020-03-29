//! Map screen where you can move

use crate::game::{GlobalState, PlayResult};
use crate::render::{Render, Drawable, Position};
use crate::game::input::Controller;

use termion::color::Color;
use termion::color;

use std::error::Error;
use std::{fs, io};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufRead;
use log::{error};
use termion::event::Key;
use std::collections::HashMap;
use std::borrow::{Borrow, BorrowMut};
use lazy_static::lazy_static;

#[derive(Clone)]
struct Tile {
    bg_color: (u8, u8, u8),
    fg_color: (u8, u8, u8),
    character: char
}

impl Tile {
    fn new(char: char) -> Tile {
        Tile {
            bg_color: (0, 200, 0),
            fg_color: (0, 0, 0),
            character: char
        }
    }

    fn bg_color(&self) -> color::Bg<color::Rgb> {
        color::Bg(color::Rgb(self.bg_color.0, self.bg_color.1, self.bg_color.2))
    }

    fn fg_color(&self) -> color::Fg<color::Rgb> {
        color::Fg(color::Rgb(self.fg_color.0, self.fg_color.1, self.fg_color.2))
    }
}

impl Drawable for Tile {
    fn draw(&self) -> String {
        unimplemented!()
    }

    fn get_width(&self) -> u16 {
        1
    }

    fn get_height(&self) -> u16 {
        1
    }

    fn get_position(&self) -> &Position {
        unimplemented!()
    }

    fn set_position(&mut self, pos: Position) {
        unimplemented!()
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    position: Position,
}

#[derive(Debug)]
struct MapParseError {
    text: String
}

impl MapParseError {
    fn new(message: &str) -> MapParseError {
        MapParseError {
            text: String::from(message)
        }
    }
}

impl Display for MapParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Failed to parse map file: {}", self.text).as_str())
    }
}

impl Error for MapParseError {

}

const MAGIC: &'static str = "MAP";
const WIDTH: u16 = 80;
const HEIGHT: u16 = 40;

const COLOR_DARK_GREEN: (u8, u8, u8) = (0, 100, 0);
const COLOR_BLACK: (u8, u8, u8) = (0, 0, 0);
const COLOR_BROWN: (u8, u8, u8) = (150, 40, 40);

lazy_static! {
    static ref DEFAULT_TILE_MAP: HashMap<char, Tile> = {
        let mut map = HashMap::new();

        map.insert('#', Tile {
            fg_color: COLOR_BLACK,
            bg_color: COLOR_DARK_GREEN,
            character: ' '
        });

        map.insert('X', Tile {
            fg_color: COLOR_BLACK,
            bg_color: COLOR_BROWN,
            character: ' '
        });

        map
    };
}

const DEFAULT_TILE: Tile = Tile {
    bg_color: COLOR_BLACK,
    fg_color: COLOR_BLACK,
    character: ' '
};

impl Map {
    fn from_file(file: &str) -> Result<Map, MapParseError> {
        let file = match File::open(file) {
            Ok(file) => file,
            Err(_) => return Err(MapParseError::new("failed to open map file")),
        };

        let lines: Vec<String> = io::BufReader::new(file).lines()
            .map(|l| l.expect("Failed to parse line"))
            .collect();

        // HEIGHT lines + 1 MAGIC LINE
        if lines.len() < (HEIGHT + 1) as usize {
            return Err(MapParseError::new("file corrupted (no magic line)"));
        }

        if !lines[0].as_str().eq(MAGIC) {
            return Err(MapParseError::new("bad magic line"));
        }

        let mut tiles = vec![];
        for line in lines.iter().skip(1) {
            if line.len() != WIDTH as usize {
                return Err(MapParseError::new(
                    format!("each row should be {} characters ({} found)!", WIDTH, line.len())
                        .as_str()))
            }

            let mut line_tiles: Vec<Tile> = vec![];
            for char in line.chars() {
                if let Some(tile) = DEFAULT_TILE_MAP.get(&char) {
                    line_tiles.push(tile.clone());
                } else {
                    line_tiles.push(DEFAULT_TILE);
                }
            }

            tiles.push(line_tiles)
        }

        Ok(Map{
            tiles,
            position: Position::from(0, 0)
        })
    }
}

pub struct PlayState {
    current_map: Map,
    need_update: bool
}

impl super::PlayState for PlayState {
    fn play(&mut self, game_state: &mut GlobalState) -> PlayResult {
        self.handle_input(&game_state.input);

        if self.need_update {
            game_state.render.clear_color = Box::new(color::Black);
            game_state.render.clear_screen();

            self.render_map(&mut game_state.render);
            self.need_update = false;
        }

        PlayResult::Still
    }

    fn to_string(&self) -> String {
        String::from("MapPlayState")
    }

    fn on_key_pressed(&mut self, game_state: &mut GlobalState, key: Key) -> PlayResult {
        PlayResult::Still
    }
}

const MAP_FOLDER: &'static str = "assets/maps/";
const MAP_START: &'static str = "start.map";

impl PlayState {
    pub fn new() -> Self {
        let start_map = Map::from_file(format!("{}{}", MAP_FOLDER, MAP_START).as_str());

        let start_map = start_map.unwrap_or_else(|err| {
            panic!("Failed to parse start map: {}", err);
        });

        PlayState {
            current_map: start_map,
            need_update: true
        }
    }

    fn handle_input(&mut self, input: &Controller) {

    }

    fn render_map(&mut self, render: &mut Render) {
        for (i, row) in self.current_map.tiles.iter().enumerate() {
            render.set_cursor_position((1, (i + 1) as u16));

            for tile in row {
                render.draw_raw(
                    format!("{}{}", tile.bg_color(), tile.character).as_str());
            }
        }
    }
}