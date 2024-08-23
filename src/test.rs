#[cfg(test)]
mod tests {
    use crate::{
        game_board::{Cordinate, GameData},
        tiles::{BlockColor, TileTypes},
    };

    fn blue_board(height: u16, width: u16) -> GameData {
        GameData::preset_new(
            height,
            width,
            vec![TileTypes::Block(BlockColor::Blue); (height * width).into()],
        )
    }

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
        let game = blue_board(4, 4);
        assert_eq!(
            game.get_cell(Cordinate::new(2, 2)).unwrap(),
            TileTypes::Block(BlockColor::Blue)
        );
    }

    #[test]
    fn out_of_bounds() {
        let game = blue_board(4, 4);
        assert!(game.get_cell(Cordinate::new(8, 8)).is_err());
    }

    #[test]
    fn out_of_bounds_assignment() {
        let mut game = blue_board(8, 8);
        assert!(game.set_cell(Cordinate::new(10, 4), TileTypes::Gap).is_err());
    }

    #[test]
    fn add_gaps() {
        let mut game = blue_board(8, 8);
        game.set_cell(Cordinate::new(4, 4), TileTypes::Gap).unwrap();
        assert_eq!(game.get_cell(Cordinate::new(4, 4)).unwrap(), TileTypes::Gap);
    }
}
