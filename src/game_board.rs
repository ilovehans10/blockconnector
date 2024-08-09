use crate::tiles::{BlockColor, TileTypes};
use termion::color;

#[derive(Debug, Clone, Copy)]
pub struct Cordinate {
    x: u16,
    y: u16,
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

    const fn in_bounds(&self, location: Cordinate) -> bool {
        location.x < self.width && location.y < self.height
    }

    pub fn get_cell(&self, location: Cordinate) -> Option<TileTypes> {
        if self.in_bounds(location) {
            Some(self.game_board[usize::from(location.x + (location.y * self.width))])
        } else {
            None
        }
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
}
