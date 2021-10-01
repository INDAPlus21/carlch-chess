# carlch-chess
# NOT WORKING CHESS ENGINE

`get_state(&self) -> GameState`
Gets current state

`set_promotion(&mut self, piece: char)`
Sets promotion

`pub struct Game {
    board: Vec<u8>,
    state: GameState,
    turn: ColorState,
    castling: Vec<Castling>,
    promotion: u8,
    enpassant: String,
}`
