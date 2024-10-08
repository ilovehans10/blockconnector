use crate::{
    game_board::{Cordinate, GameData, Shape, ShapeType},
    tiles,
};

pub fn game_loop(height: u16, width: u16) {
    let mut game = GameData::new(height, width);
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
