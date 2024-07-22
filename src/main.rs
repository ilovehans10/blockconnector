mod tiles;
use crate::tiles::{BlockColor, TileTypes};
use termion::color;

const HEIGHT: u16 = 8;
const WIDTH: u16 = 16;

fn main() {
    let mut game_board = vec![TileTypes::new(); (HEIGHT * WIDTH).into()];
    game_board.fill_with(|| TileTypes::Block(rand::random::<BlockColor>()));
    println!("╔{}╗", "═".repeat((WIDTH * 2).into()));
    for current_row in game_board.chunks(WIDTH.into()) {
        print!("║");
        for current_element in current_row {
            print!("{current_element}");
        }
        println!("{}║", color::Bg(color::Reset));
    }
    println!("╚{}╝", "═".repeat((WIDTH * 2).into()));
}
