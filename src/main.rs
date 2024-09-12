use game_board::{Cordinate, GameData, Shape, ShapeType};
mod game_board;
mod tiles;

#[cfg(test)]
mod test;

const HEIGHT: u16 = 8;
const WIDTH: u16 = 16;

fn main() {
    let mut game = GameData::new(HEIGHT, WIDTH);
    let rectangle = Shape::new(&ShapeType::Rectangle(Cordinate::new(8, 4)));
    game.draw_info();
    println!();
    game.apply_shape(&rectangle, Cordinate::new(4, 2), tiles::TileTypes::Gap)
        .unwrap();
    game.set_cell(Cordinate::new(4, 0), tiles::TileTypes::Empty)
        .unwrap();
    game.draw_info();
    println!();
    game.draw_board();
}
