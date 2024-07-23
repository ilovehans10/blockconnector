use game_board::Cordinate;

use crate::game_board::GameData;
mod game_board;
mod tiles;

const HEIGHT: u16 = 8;
const WIDTH: u16 = 16;

fn main() {
    let game = GameData::new(HEIGHT, WIDTH);
    println!("{}", game.get_cell(Cordinate::new(0, 0)).unwrap());
    println!("{}", game.get_cell(Cordinate::new(1, 1)).unwrap());
    println!("{}", game.get_cell(Cordinate::new(4, 7)).unwrap());
    game.draw_raw();
    game.draw_board();
    game.draw_board();
}
