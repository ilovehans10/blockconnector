#[cfg(test)]
mod tests {
    use crate::{
        game_board::{Cordinate, GameData},
        tiles::{BlockColor, TileTypes},
    };

    #[test]
    fn make_board() {
        GameData::new(8, 8);
    }
    #[test]
    fn make_debug_board() {
        GameData::preset_new(8, 8, vec![TileTypes::Block(BlockColor::Blue)]);
    }
    #[test]
    fn print_raw_board() {
        GameData::preset_new(8, 8, vec![TileTypes::Block(BlockColor::Blue)]).draw_board();
    }
    #[test]
    fn get_cell_check() {
        let game = GameData::preset_new(4, 4, vec![TileTypes::Block(BlockColor::Blue)]);
        assert_eq!(
            game.get_cell(Cordinate::new(0, 0)).unwrap(),
            TileTypes::Block(BlockColor::Blue)
        );
    }
}
