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
    // Create new standard board
    fn new() -> Game {
        let mut game = Game {
            board: Vec::new(),
            state: GameState::InProgress,
        };
        game.apply_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        game
    }

    // Create custom board
    fn new_board(fen: &str) -> Game {
        let mut game = Game {
            board: Vec::new(),
            state: GameState::InProgress,
        };
        game.apply_fen(fen);
        game
    } 

    // Take in fen string and apply the board state to the game
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
                        'K' => holder.push(0b0001_0001),
                        'k' => holder.push(0b0000_1001),
                        'Q' => holder.push(0b0001_0110),
                        'q' => holder.push(0b0000_1110),
                        'B' => holder.push(0b0001_0101),
                        'b' => holder.push(0b0000_1101),
                        'N' => holder.push(0b0001_0100),
                        'n' => holder.push(0b0000_1100),
                        'R' => holder.push(0b0001_0011),
                        'r' => holder.push(0b0000_1011),
                        'P' => holder.push(0b0001_0010),
                        'p' => holder.push(0b0000_1010),
                        _ => {
                            for _ in 0..x.to_digit(10).unwrap() as usize {
                                holder.push(0b0000_0000);
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

    // Function to get vector index as a grid position
    fn vector_to_grid(input: u8) -> String {
        let row = input / 8;
        let column = input % 8;

        // Get row as letter
        let mut row_string: String = match row {
            0 => "A",
            1 => "B",
            2 => "C",
            3 => "D",
            4 => "E",
            5 => "F",
            6 => "G",
            7 => "H",
            _ => " ",
        }.to_owned();
        
        // Get column as number
        let col_string: &str = match column {
            0 => "1",
            1 => "2",
            2 => "3",
            3 => "4",
            4 => "5",
            5 => "6",
            6 => "7",
            7 => "8",
            _ => " ",
        };

        // Add column to row string
        row_string.push_str(&col_string);
        row_string // Return position
    }

    // Get all valid moves possible by each piece
    fn get_valid_moves(&self, tile: &str) -> Option<Vec<u8>> {
        let mut x: usize = 0;
        let mut y: usize = 0;

        for index in tile.chars() {
            match index {
                // Y-coordinate
                'A' => {y = 0},
                'B' => {y = 1},
                'C' => {y = 2},
                'D' => {y = 3},
                'E' => {y = 4},
                'F' => {y = 5},
                'G' => {y = 6},
                'H' => {y = 7},
            
                // X-coordinate
                '1' => {x = 0},
                '2' => {x = 1},
                '3' => {x = 2},
                '4' => {x = 3},
                '5' => {x = 4},
                '6' => {x = 5},
                '7' => {x = 6},
                '8' => {x = 7},
                
                // No valid input
                _ => {
                    y = 0;
                    x = 0;
                },
            }
        }

        let selected_piece = &self.board[(y * 8) + x];
        let mut possible_moves: Vec<u8> = Vec::new();
        // Return possible moves
        return match selected_piece {
            // King piece
            0b0001_0001 | 0b0000_1001 => Some(self.get_surrounding_tiles(y * 8 + x, true, true, false)),
            // Queen piece
            0b0001_0110 | 0b0000_1110 => Some(self.get_surrounding_tiles(y * 8 + x, true, true, true)),
            // Bishop piece
            0b0001_0101 | 0b0000_1101 => Some(self.get_surrounding_tiles(y * 8 + x, true, false, true)),   
            // Knight piece
            0b0001_0100 | 0b0000_1100 => None,
            // Rook piece
            0b0001_0011 | 0b0000_1011 => Some(self.get_surrounding_tiles(y * 8 + x, false, true, true)),
            // Pawn piece
            0b0001_0010 | 0b0000_1010 => Some(self.get_pawn_moves(y * 8 + x)), 
            _ => None,
        }
    }

    // Get surrounding valid tiles
    fn get_surrounding_tiles(&self, index: usize, is_diagonal: bool, is_linear: bool, is_continous: bool) -> Vec<u8> {
        let mut tile_check = if is_linear { 0 } else { 1 };
        let mut available_tiles: Vec<u8> = Vec::new();

        // Empty tile will not have piece on top
        if self.board[index] == 0b0000_0000 {
            return available_tiles;
        }
        // Check all 8 surrounding tiles if piece can move 8 directions
        // Otherwise check 4 surrounding tiles
        while tile_check < 8 {
            match tile_check {
                // NORTH
                0 => {
                    for continous_loop in 1..8 {
                        if index as isize - (8 * continous_loop as isize) >= 0 {
                            if &self.board[index - (8 * continous_loop)] >> 3 | &self.board[index] >> 3 == 0b0000_0011 {
                                available_tiles.push((index - ((8 * continous_loop) as usize)) as u8); 
                                break;
                            } else if &self.board[index - (8 * continous_loop)] >> 3 ^ &self.board[index] >> 3 != 0b0000_0000 {
                                available_tiles.push((index - ((8 * continous_loop) as usize)) as u8);
                            } else {
                                break;
                            }   
                        } else {
                            break;
                        }   
                        if !is_continous {
                            break;
                        }
                    }
                }
                // NORTH EAST
                1 => {
                    for continous_loop in 1..8 {
                        if index as isize - (8 * continous_loop as isize) >= 0 {
                            if &self.board[index - (8 + 1) * continous_loop] >> 3 | &self.board[index] >> 3 == 0b0000_0011 && ((index % 8) + 1 * continous_loop) < 8 {
                                available_tiles.push((index - ((8 + 1) * continous_loop) as usize) as u8); 
                                break;
                            } else if &self.board[index - (8 + 1) * continous_loop] >> 3 ^ &self.board[index] >> 3 != 0b0000_0000 && ((index % 8) + 1 * continous_loop) < 8 {
                                available_tiles.push((index - ((8 + 1) * continous_loop) as usize) as u8); 
                            } else {
                                break;
                            }   
                        } else {
                            break;
                        }
                        if !is_continous {
                            break;
                        }
                    }  
                }
                // EAST
                2 => {
                    for continous_loop in 1..8 {
                        if index + (1 * continous_loop) < 644 && &self.board[index + (1 * continous_loop)] >> 3 | &self.board[index] >> 3 == 0b0000_0011 && ((index % 8) + 1 * continous_loop) < 8 {
                            available_tiles.push((index + (1 * continous_loop) as usize) as u8);
                            break;
                        } else if index + (1 * continous_loop) < 64 && &self.board[index + (1 * continous_loop)] >> 3 ^ &self.board[index] >> 3 != 0b0000_0000 && ((index % 8) + 1 * continous_loop) < 8 {
                            available_tiles.push((index + (1 * continous_loop) as usize) as u8);
                        } else {
                            break;
                        }
                        if !is_continous {
                            break;
                        }
                    }
                }
                // SOUTH EAST
                3 => {
                    for continous_loop in 1..8 {
                        if index + ((8 + 1) * continous_loop) < 64 && &self.board[index + (8 + 1) * continous_loop] >> 3 | &self.board[index] >> 3 == 0b0000_0011 && ((index % 8) + 1 * continous_loop) < 8 {
                            available_tiles.push((index + ((8 + 1) * continous_loop) as usize) as u8);
                            break;
                        } else if index + ((8 + 1) * continous_loop) < 64 && &self.board[index + (8 + 1) * continous_loop] >> 3 ^ &self.board[index] >> 3 != 0b0000_0000 && ((index % 8) + 1 * continous_loop) < 8 {
                            available_tiles.push((index + ((8 + 1) * continous_loop) as usize) as u8);
                        } else {
                            break;
                        }
                        if !is_continous {
                            break;
                        }
                    }
                }
                // SOUTH
                4 => {
                    for continous_loop in 1..8 {
                        if index + (8 * continous_loop) < 64 && &self.board[index + (8 * continous_loop)] >> 3 | &self.board[index] >> 3 == 0b0000_0011 {
                            available_tiles.push((index + (8 * continous_loop) as usize) as u8);
                            break;
                        } else if index + (8 * continous_loop) < 64 && &self.board[index + (8 * continous_loop)] >> 3 ^ &self.board[index] >> 3 != 0b0000_0000 {
                            available_tiles.push((index + (8 * continous_loop) as usize) as u8);
                        } else {
                            break;
                        }   
                        if !is_continous {
                            break;
                        }
                    }
                }
                // SOUTH WEST
                5 => {
                    for continous_loop in 1..8 {
                        if index + (8 - 1) * continous_loop < 64 && &self.board[index + ((8 - 1) * continous_loop)] >> 3 | &self.board[index] >> 3 == 0b0000_0011 && (index % 8) as isize - continous_loop as isize >= 0 {
                            available_tiles.push((index + (8 - 1) * continous_loop) as u8);
                            break;
                        } else if index + (8 - 1) * continous_loop < 64 && &self.board[index + ((8 - 1) * continous_loop)] >> 3 ^ &self.board[index] >> 3 != 0b0000_0000 && (index % 8) as isize - continous_loop as isize >= 0 {
                            available_tiles.push((index + (8 - 1) * continous_loop) as u8);
                        } else {
                            break;
                        }
                        if !is_continous {
                            break;
                        }
                    }
                }
                // WEST
                6 => {
                    for continous_loop in 1..8 {
                        if index as isize - (1 * continous_loop as isize) >= 0 {
                            if &self.board[index - (1 * continous_loop)] >> 3 | &self.board[index] >> 3 == 0b0000_0011 && (index % 8) as isize- continous_loop as isize >= 0 {
                                available_tiles.push((index - 1 * continous_loop) as u8);
                                break;
                            } else if &self.board[index - (1 * continous_loop)] >> 3 ^ &self.board[index] >> 3 != 0b0000_0000 && (index % 8) as isize - continous_loop as isize >= 0 { 
                                available_tiles.push((index - 1 * continous_loop) as u8);
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                        if !is_continous {
                            break;
                        }
                    }
                }
                // NORTH WEST
                7 => {
                    for continous_loop in 1..8 {
                        if index as isize - (8 + 1) as isize * continous_loop as isize >= 0 && &self.board[index - ((8 + 1) * continous_loop)] >> 3 | &self.board[index] >> 3 == 0b0000_0011 && (index % 8) as isize - continous_loop as isize >= 0 {
                            available_tiles.push((index - (8 + 1) * continous_loop) as u8);
                            break;
                        } else if index as isize - (8 + 1) as isize * continous_loop as isize >= 0 && &self.board[index - ((8 + 1) * continous_loop)] >> 3 ^ &self.board[index] >> 3 != 0b0000_0000 && (index % 8) as isize - continous_loop as isize >= 0 {
                            available_tiles.push((index - (8 + 1) * continous_loop) as u8);
                        } else {
                            break;
                        }
                        if !is_continous {
                            break;
                        }
                    }  
                }
                _ => { }
            } 

            if is_diagonal { tile_check += 1; }
            else { tile_check += 2; }
        }
        available_tiles
    }

    fn get_knight_moves(&self, index: usize) -> Vec<u8> {
        let mut tile_check = 0;
        let mut available_moves: Vec<u8> = Vec::new();
        while(tile_check < 8) {
            match tile_check {
                // Up, Left
                0 => {},
                // Up, Right
                1 => {},
                // Right, Up
                2 => {},
                // Right, Down
                3 => {},
                // Down, Left
                4 => {},
                // Down, Right
                5 => {},
                // Left, Up
                6 => {},
                // Left, Down
                7 => {},
                // (?)
                _ => {},
            }   

            tile_check += 1;
        }
    }

    // Get possible pawn moves
    fn get_pawn_moves(&self, index: usize) -> Vec<u8> {
        let mut available_moves: Vec<u8> = Vec::new();
        match self.board[index] {
            // For black pieces
            0b0000_1010 => {
                if self.board[index] >= 8 && self.board[index] < 16 as u8 {
                    available_moves.push(index as u8 + 8);
                    available_moves.push(index as u8 + 16);
                } else if index + 8 > 63 {
                    available_moves.push(index as u8 + 8);
                }
            },
            // For white pieces
            0b0001_0010 => {
                if self.board[index] >= 56 && self.board[index] < 64 as u8 {
                    available_moves.push(index as u8 - 8);
                    available_moves.push(index as u8 - 16);
                } else if index as isize - 8 <= 0 {
                    available_moves.push(index as u8 - 8);
                } 
            }, 
            _ => { },
        }
        available_moves
    }

    // Print out the curent board and pieces
    fn display_board(&self) {
        let current_board = &self.board;
        let mut column = 8;
        print!("   ");
        for index in 0..8 {
            print!("{} ", index + 1);
        }
        let mut row = 0;
        for index in current_board {
            if column == 8 {
                row = row + 1;
                column = 0;
                print!("\n{}  ", match row {
                    1 => "A",
                    2 => "B",
                    3 => "C",
                    4 => "D",
                    5 => "E",
                    6 => "F",
                    7 => "G",
                    8 => "H",
                    _ => " ",
                });
            }
            match index {
                0b0001_0001 => print!("K "),
                0b0000_1001 => print!("k "),
                0b0001_0110 => print!("Q "),
                0b0000_1110 => print!("q "),
                0b0001_0101 => print!("B "),
                0b0000_1101 => print!("b "),
                0b0001_0100 => print!("N "),
                0b0000_1100 => print!("n "),
                0b0001_0011 => print!("R "),
                0b0000_1011 => print!("r "),
                0b0001_0010 => print!("P "),
                0b0000_1010 => print!("p "),
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
    fn init_test() {
        let a = Game::new();
    }

    #[test]
    fn validator_test() {
        let a = Game::new_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        a.display_board();
    }   
} 
