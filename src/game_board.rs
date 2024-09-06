use crate::tiles::{BlockColor, TileTypes};
use std::ops::{Add, Sub};
use itertools::Itertools;
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
}

pub enum ShapeType {
    Rectangle(Cordinate),
}

#[derive(Clone, Copy)]
struct AdjacentData {
    up: Option<bool>,
    down: Option<bool>,
    left: Option<bool>,
    right: Option<bool>,
}

// in this case adjacent doesn't include diagnols and in other words is in reference to cells with
// a manhatan distance of one
pub struct GameData {
    game_board: Vec<TileTypes>,
    adjacent_cache: Vec<Option<AdjacentData>>,
    _turn_number: u8,
    _max_turns: u8,
    height: u16,
    width: u16,
}

impl Cordinate {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    fn checked_add_signed(&self, rhs_x: i16, rhs_y: i16) -> Option<Self> {
        let x = self.x.checked_add_signed(rhs_x)?;
        let y = self.y.checked_add_signed(rhs_y)?;
        Some(Self { x, y })
    }

    pub fn adjacent(&self) -> Vec<Option<Cordinate>> {
        let mut adjacent_cords = Vec::with_capacity(4);
        for rhs_x in [-1, 1] {
            for rhs_y in [-1, 1] {
                adjacent_cords.push(self.checked_add_signed(rhs_x, rhs_y));
            }
        }
        adjacent_cords
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
    pub fn new(shape_type: &ShapeType) -> Self {
        match shape_type {
            ShapeType::Rectangle(size) => {
                let volume = usize::from(size.x * size.y);
                let mut locations = Vec::<Cordinate>::with_capacity(volume);
                for (x, y) in (0..size.x).cartesian_product(0..size.y) {
                    locations.push(Cordinate::new(x, y));
                }
                Self { locations }
            }
        }
    }
}

impl GameData {
    pub fn new(height: u16, width: u16) -> Self {
        let size = usize::from(height * width);
        let mut prototype_board = vec![TileTypes::new(); size];
        prototype_board.fill_with(|| TileTypes::Block(rand::random::<BlockColor>()));
        Self {
            game_board: prototype_board,
            adjacent_cache: vec![None; size],
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
        let size = usize::from(height * width);
        Self {
            game_board: board,
            adjacent_cache: vec![None; size],
            _turn_number: 0,
            _max_turns: 0,
            height,
            width,
        }
    }

    const fn in_bounds(&self, location: Cordinate) -> Result<(), BoardError> {
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
        self.in_bounds(location)?;
        Ok(self.game_board[usize::from(location.x + (location.y * self.width))])
    }

    pub fn set_cell(
        &mut self,
        location: Cordinate,
        tile_type: TileTypes,
    ) -> Result<(), BoardError> {
        self.in_bounds(location)?;
        let location_index = usize::from(location.x + (location.y * self.width));
        self.adjacent_cache[location_index] = None;
        self.game_board[location_index] = tile_type;
        Ok(())
    }

    fn all_cords(&self) -> Vec<Cordinate> {
        let length = usize::from(self.height * self.width);
        let mut all_cordinate_vector = Vec::with_capacity(length);
        for (y, x) in (0..self.height).cartesian_product(0..self.width) {
            all_cordinate_vector.push(Cordinate::new(x, y));
        }
        all_cordinate_vector
    }

    fn check_adjacent_cache(&self) -> bool {
        self.adjacent_cache.iter().any(|&x| x.is_none())
    }

    fn update_adjacent_cache(&mut self) {
        todo!("Make adjacent cache update function");
    }

    pub fn draw_raw(&self) {
        self.game_board.iter().for_each(|x| print!("{x:?}"));
        println!();
    }

    pub fn draw_info(&self) {
        self.game_board
            .chunks(self.width.into())
            .rev()
            .enumerate()
            .for_each(|(index, tile_chunk)| {
                let rev_index = usize::from(self.height - 1) - index;
                print!("{rev_index:2}");
                for tile in tile_chunk {
                    print!("{tile:?} ");
                }
                if index < (usize::from(self.height) - 1) {
                    println!("\n");
                } else {
                    println!();
                };
            });
        print!(" ");
        (0..self.width).for_each(|index| print!("{index:3?}"));
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
        self.in_bounds(offset + *shape.locations.last().unwrap())?;
        for location_item in &shape.locations {
            let location = *location_item + offset;
            self.set_cell(location, tile_type)?;
        }
        Ok(())
    }
}
