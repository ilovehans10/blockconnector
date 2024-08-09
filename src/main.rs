use game_board::{Cordinate, GameData};
mod game_board;
mod tiles;

#[cfg(test)]
mod test;

const HEIGHT: u16 = 8;
const WIDTH: u16 = 16;

fn main() {
    let mut game = GameData::new(HEIGHT, WIDTH);
    println!("{:?}", game.get_cell(Cordinate::new(0, 0)).unwrap());
    println!("{:?}", game.get_cell(Cordinate::new(1, 1)).unwrap());
    game.set_cell(Cordinate::new(4, 7), tiles::TileTypes::Gap)
        .unwrap();
    game.draw_raw();
    game.draw_board();
}
