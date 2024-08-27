use crate::tiles::{BlockColor, TileTypes};
use std::ops::{Add, Sub};
use termion::color;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct Cordinate {
    x: u16,
    y: u16,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum BoardError {
    #[error("expected within {limit}, but found {exception}")]
    BoundsError { limit: u16, exception: u16 },
}

pub struct Shape {
    locations: Vec<Cordinate>,
    shape_type: ShapeType,
}

pub enum ShapeType {
    Rectangle(Cordinate),
}

pub struct GameData {
    game_board: Vec<TileTypes>,
    _turn_number: u8,
    _max_turns: u8,
    height: u16,
    width: u16,
}

impl Cordinate {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl Add for Cordinate {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Cordinate {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Shape {
    pub fn new(shape_type: ShapeType) -> Self {
        match shape_type {
            ShapeType::Rectangle(size) => {
                let mut locations = Vec::<Cordinate>::with_capacity((size.x * size.y).into());
                for x in 0..size.x {
                    for y in 0..size.y {
                        locations.push(Cordinate::new(x, y));
                    }
                }
                Self {
                    locations,
                    shape_type,
                }
            }
        }
    }
}

impl GameData {
    pub fn new(height: u16, width: u16) -> Self {
        let mut prototype_board = vec![TileTypes::new(); (height * width).into()];
        prototype_board.fill_with(|| TileTypes::Block(rand::random::<BlockColor>()));
        Self {
            game_board: prototype_board,
            _turn_number: 0,
            _max_turns: 0,
            height,
            width,
        }
    }

    // Consider passing a function for tile generation to allow for no randomness
    // This would possibly require adding a function to the struct for generating new cells
    #[cfg(test)]
    pub fn preset_new(height: u16, width: u16, board: Vec<TileTypes>) -> Self {
        Self {
            game_board: board,
            _turn_number: 0,
            _max_turns: 0,
            height,
            width,
        }
    }

    const fn in_bounds(&self, location: &Cordinate) -> Result<(), BoardError> {
        if location.x >= self.width {
            return Err(BoardError::BoundsError {
                limit: self.width,
                exception: location.x,
            });
        }
        if location.y >= self.height {
            return Err(BoardError::BoundsError {
                limit: self.height,
                exception: location.y,
            });
        }
        Ok(())
    }

    pub fn get_cell(&self, location: Cordinate) -> Result<TileTypes, BoardError> {
        self.in_bounds(&location)?;
        Ok(self.game_board[usize::from(location.x + (location.y * self.width))])
    }

    pub fn set_cell(
        &mut self,
        location: &Cordinate,
        tile_type: TileTypes,
    ) -> Result<(), BoardError> {
        self.in_bounds(location)?;
        self.game_board[usize::from(location.x + (location.y * self.width))] = tile_type;
        Ok(())
    }

    pub fn draw_raw(&self) {
        self.game_board.iter().for_each(|x| print!("{x:?}"));
        println!();
    }

    pub fn draw_board(&self) {
        println!("╔{}╗", "═".repeat((self.width * 2).into()));
        for current_row in self.game_board.chunks(self.width.into()).rev() {
            print!("║");
            for current_element in current_row {
                print!("{current_element}");
            }
            println!("{}║", color::Bg(color::Reset));
        }
        println!("╚{}╝", "═".repeat((self.width * 2).into()));
    }

    pub fn apply_shape(
        &mut self,
        shape: &Shape,
        offset: Cordinate,
        tile_type: TileTypes,
    ) -> Result<(), BoardError> {
        self.in_bounds(&(offset + *shape.locations.last().unwrap()))?;
        for location_item in &shape.locations {
            let location = *location_item + offset;
            self.set_cell(&location, tile_type)?;
        }
        Ok(())
    }
}
