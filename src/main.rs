use crate::game_board::GameData;
mod game_board;
mod tiles;

const HEIGHT: u16 = 8;
const WIDTH: u16 = 16;

fn main() {
    let game = GameData::new(HEIGHT, WIDTH);
    game.draw_raw();
    game.draw_board();
    game.draw_board();
}
