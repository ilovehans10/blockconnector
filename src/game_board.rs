use crate::tiles::{BlockColor, TileTypes};
use termion::color;

pub struct GameData {
    game_board: Vec<TileTypes>,
    _turn_number: u8,
    _max_turns: u8,
    _height: u16,
    width: u16,
}

impl GameData {
    pub fn new(height: u16, width: u16) -> Self {
        let mut prototype_board = vec![TileTypes::new(); (height * width).into()];
        prototype_board.fill_with(|| TileTypes::Block(rand::random::<BlockColor>()));
        Self {
            game_board: prototype_board,
            _turn_number: 0,
            _max_turns: 0,
            _height: height,
            width,
        }
    }
    pub fn draw_raw(&self) {
        self.game_board.iter().for_each(|x| print!("{x}"));
        println!("{}", color::Bg(color::Reset));
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
