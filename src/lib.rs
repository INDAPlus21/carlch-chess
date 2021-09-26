// Chess library
use std::fmt;
use std::iter::Iterator;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]

// GameState represents the current state of the game
pub enum GameState {
    InProgress,
    Check,
    GameOver
    // Checkmate
    // DeadPosition
}

// 
pub struct Game {
    board: Vec<u8>,
    state: GameState,
}

impl Game {
    fn new() -> Game {
        let mut game = Game {
            board: Vec::new(),
            state: GameState::InProgress,
        };
        game.apply_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        game
    }

    // Take in fen string and apply the board state to the game
    // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    fn apply_fen(&mut self, fen: &str) {
        // Translate FEN string instruction
        let fen_checker: Vec<String> = fen.split(" ").map(|_check| _check.to_string()).collect::<Vec<String>>();

        // Get board positions
        // (fen_checker[0])
        self.board = {
            fen_checker[0].split("/").map(|_char| {
                let mut holder: Vec<u8> = Vec::new();
                for x in _char.chars() {
                    match x {
                        'K' => holder.push(0b0001),
                        'k' => holder.push(0b1001),
                        'Q' => holder.push(0b0110),
                        'q' => holder.push(0b1110),
                        'B' => holder.push(0b0101),
                        'b' => holder.push(0b1101),
                        'N' => holder.push(0b0100),
                        'n' => holder.push(0b1100),
                        'R' => holder.push(0b0011),
                        'r' => holder.push(0b1011),
                        'P' => holder.push(0b0010),
                        'p' => holder.push(0b1010),
                        _ => {
                            for _ in 0..x.to_digit(10).unwrap() as usize {
                                holder.push(0b0000);
                            }           
                        },
                    }   
                };
                return holder
            }).flatten().collect::<Vec<u8>>()
        };

        // Get turn
        // (fen_checker[1])

        // Get castling availability
        // (fen_checker[2])

        // Get en passent availability
        // (fen_checker[3])

        // Halfmove clock
        // (fen_checker[4])
        
        // Fullmove number
        // (fen_checker[5])
    }

    fn get_valid_moves(&self, ) {

    }

    // Print out the curent board and pieces
    fn display_board(&self) {
        let current_board = &self.board;
        let mut column = 8;
        print!("   ");
        for index in 0..8 {
            print!("{} ", index);
        }
        for index in current_board {
            let mut row = 0;
            if column == 8 {
                row = row + 1;
                column = 0;
                print!("\n{}  ", row);
            }
            match index {
                0b0001 => print!("K "),
                0b1001 => print!("k "),
                0b0110 => print!("Q "),
                0b1110 => print!("q "),
                0b0101 => print!("B "),
                0b1101 => print!("b "),
                0b0100 => print!("N "),
                0b1100 => print!("n "),
                0b0011 => print!("R "),
                0b1011 => print!("r "),
                0b0010 => print!("P "),
                0b1010 => print!("p "),
                _ => print!("  "),
            }
            column = column + 1;
        }
    }
}


    
#[cfg(test)]
mod tests {
    use super::*;
    // Testing WIP
    #[test]
    fn test() {
        let a = Game::new();
        a.display_board();
    }
} 
