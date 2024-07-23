use crate::tiles::{BlockColor, TileTypes};
use termion::color;

pub struct GameData {
    game_board: Vec<TileTypes>,
    turn_number: u8,
    max_turns: u8,
    height: u16,
    width: u16,
}

impl GameData {
    pub fn new(height: u16, width: u16) -> Self {
        let mut prototype_board = vec![TileTypes::new(); (height * width).into()];
        prototype_board.fill_with(|| TileTypes::Block(rand::random::<BlockColor>()));
        Self {
            game_board: prototype_board,
            turn_number: 0,
            max_turns: 0,
            height,
            width,
        }
    }
    pub fn draw_board(&self) {
        println!("╔{}╗", "═".repeat((self.width * 2).into()));
        for current_row in self.game_board.chunks(self.width.into()) {
            print!("║");
            for current_element in current_row {
                print!("{current_element}");
            }
            println!("{}║", color::Bg(color::Reset));
        }
        println!("╚{}╝", "═".repeat((self.width * 2).into()));
    }
}
