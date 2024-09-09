use crate::tiles::{BlockColor, TileTypes};
use itertools::Itertools;
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

    pub fn adjacent(&self) -> Vec<Option<Self>> {
        let mut adjacent_cords = Vec::with_capacity(4);
        for (rhs_1, rhs_2) in std::iter::once(0).cartesian_product([1, -1]) {
            adjacent_cords.push(self.checked_add_signed(rhs_1, rhs_2));
            adjacent_cords.push(self.checked_add_signed(rhs_2, rhs_1));
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

    fn cordinates_to_index(&self, location: Cordinate) -> Result<usize, BoardError> {
        self.in_bounds(location)?;
        Ok(usize::from(location.x + (location.y * self.width)))
    }

    pub fn get_cell(&self, location: Cordinate) -> Result<TileTypes, BoardError> {
        Ok(self.game_board[self.cordinates_to_index(location)?])
    }

    pub fn set_cell(
        &mut self,
        location: Cordinate,
        tile_type: TileTypes,
    ) -> Result<(), BoardError> {
        let location_index = self.cordinates_to_index(location)?;
        self.adjacent_cache[location_index] = None;
        for adjacent_location in location.adjacent().into_iter().flatten() {
            if self.in_bounds(adjacent_location).is_ok() {
                let adjacent_location_index = self.cordinates_to_index(adjacent_location)?;
                self.adjacent_cache[adjacent_location_index] = None;
            }
        }
        self.game_board[location_index] = tile_type;
        self.update_adjacent_cache()?;
        Ok(())
    }

    fn get_adjacency_status(
        &self,
        location: Cordinate,
    ) -> Result<Option<AdjacentData>, BoardError> {
        Ok(self.adjacent_cache[self.cordinates_to_index(location)?])
    }

    fn all_cords(&self) -> Vec<Cordinate> {
        let length = usize::from(self.height * self.width);
        let mut all_cordinate_vector = Vec::with_capacity(length);
        for (y, x) in (0..self.height).cartesian_product(0..self.width) {
            all_cordinate_vector.push(Cordinate::new(x, y));
        }
        all_cordinate_vector
    }

    fn adjacent_cache_is_dirty(&self) -> bool {
        self.adjacent_cache.iter().any(|&x| x.is_none())
    }

    fn reset_adjacent_cache(&mut self) {
        self.adjacent_cache.fill(None);
    }

    fn update_adjacent_cache(&mut self) -> Result<(), BoardError> {
        for (index, top_cordinates) in self.all_cords().into_iter().enumerate() {
            if self.get_adjacency_status(top_cordinates).unwrap().is_some() {
                continue;
            }

            let (mut up, mut down, mut left, mut right) =
                (Some(false), Some(false), Some(false), Some(false));
            for (direction, adjacent_cordinates) in
                top_cordinates.adjacent().into_iter().enumerate()
            {
                let Some(real_adjacent_cords) = adjacent_cordinates else {
                    continue;
                };
                if self.in_bounds(real_adjacent_cords).is_err() {
                    continue;
                }
                if self.get_cell(top_cordinates)? != self.get_cell(real_adjacent_cords)? {
                    continue;
                }
                match direction {
                    0 => up = Some(true),
                    1 => right = Some(true),
                    2 => down = Some(true),
                    3 => left = Some(true),
                    _ => panic!("There should only be 4 adjacency values, but a 5th was found"),
                }
            }
            self.adjacent_cache[index] = Some(AdjacentData {
                up,
                down,
                left,
                right,
            });
        }
        Ok(())
    }

    pub fn draw_raw(&self) {
        self.game_board.iter().for_each(|x| print!("{x:?}"));
        println!();
    }

    pub fn draw_info(&self) {
        self.game_board
            .iter()
            .zip(self.adjacent_cache.iter())
            .collect::<Vec<_>>()
            .chunks(self.width.into())
            .rev()
            .enumerate()
            .for_each(|(index, cell_data_chunk)| {
                let rev_index = usize::from(self.height - 1) - index;
                print!("{rev_index:2}");
                for (tile, adjacency) in cell_data_chunk {
                    let (left_string, right_string) = match adjacency {
                        Some(AdjacentData {
                            up: _,
                            down: _,
                            left,
                            right,
                        }) => match (left, right) {
                            (None, None) => (String::from("|"), String::from("|")),
                            (None, Some(true)) => (String::from("|"), String::from("=")),
                            (None, Some(false)) => (String::from("|"), String::from(":")),
                            (Some(true), None) => (String::from("="), String::from("|")),
                            (Some(false), None) => (String::from(":"), String::from("|")),
                            (Some(true), Some(true)) => (String::from("="), String::from("=")),
                            (Some(false), Some(true)) => (String::from(":"), String::from("=")),
                            (Some(true), Some(false)) => (String::from("="), String::from(":")),
                            (Some(false), Some(false)) => (String::from(":"), String::from(":")),
                        },
                        None => (String::from("X"), String::from("X")),
                    };
                    print!("{left_string}{tile:?}{right_string}");
                }
                println!();
                if index < (usize::from(self.height) - 1) {
                    print!("{index} ");
                    for column in 0..self.width {
                        let upper_cell_index = (rev_index*usize::from(self.width)) + usize::from(column);
                        let lower_cell_index = upper_cell_index - usize::from(self.width);
                        let upper_cell_adjacency = match self.adjacent_cache.get(upper_cell_index).unwrap() {
                            Some(AdjacentData { up: _, down: x, left: _, right: _ }) => {
                                match x {
                                    Some(true) => String::from("|"),
                                    Some(false) => String::from(":"),
                                    None => panic!("There should be no none values inbetween rows"),
                                }
                            },
                            None => String::from("X"),
                        };
                        let lower_cell_adjacency = match self.adjacent_cache.get(lower_cell_index).unwrap() {
                            Some(AdjacentData { up: x, down: _, left: _, right: _ }) => {
                                match x {
                                    Some(true) => String::from("|"),
                                    Some(false) => String::from(":"),
                                    None => panic!("There should be no none values inbetween rows"),
                                }
                            },
                            None => String::from("X"),
                        };
                        print!(" ");
                        print!("{upper_cell_adjacency}{lower_cell_adjacency}");
                        print!(" ");
                    }
                    println!();
                };
            });
        print!(" ");
        (0..self.width).for_each(|index| print!("{index:4?}"));
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
