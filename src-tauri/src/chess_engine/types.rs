use serde::{Deserialize, Serialize};
use crate::chess_engine::error::{ChessError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Square {
    index: u8,
}

impl Square {
    pub fn new(index: u8) -> Option<Square> {
        if index < 64 {
            Some(Square { index })
        } else {
            None
        }
    }

    pub fn from_algebraic(s: &str) -> Result<Square> {
        if s.len() != 2 {
            return Err(ChessError::InvalidSquare {
                square: s.to_string(),
            });
        }

        let chars: Vec<char> = s.chars().collect();
        let file = match chars[0] {
            'a'..='h' => (chars[0] as u8) - b'a',
            _ => {
                return Err(ChessError::InvalidSquare {
                    square: s.to_string(),
                })
            }
        };

        let rank = match chars[1] {
            '1'..='8' => (chars[1] as u8) - b'1',
            _ => {
                return Err(ChessError::InvalidSquare {
                    square: s.to_string(),
                })
            }
        };

        Ok(Square {
            index: rank * 8 + file,
        })
    }

    pub fn to_algebraic(&self) -> String {
        let file = (b'a' + (self.index % 8)) as char;
        let rank = (b'1' + (self.index / 8)) as char;
        format!("{}{}", file, rank)
    }

    pub fn rank(&self) -> u8 {
        self.index / 8
    }

    pub fn file(&self) -> u8 {
        self.index % 8
    }

    pub fn from_rank_file(rank: u8, file: u8) -> Option<Square> {
        if rank < 8 && file < 8 {
            Some(Square {
                index: rank * 8 + file,
            })
        } else {
            None
        }
    }

    pub fn index(&self) -> u8 {
        self.index
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Piece>,
    pub is_castling: bool,
    pub is_en_passant: bool,
}

impl Move {
    pub fn new(from: Square, to: Square) -> Self {
        Move {
            from,
            to,
            promotion: None,
            is_castling: false,
            is_en_passant: false,
        }
    }

    pub fn to_uci(&self) -> String {
        let mut uci = format!("{}{}", self.from.to_algebraic(), self.to.to_algebraic());
        if let Some(promotion) = self.promotion {
            let promo_char = match promotion {
                Piece::Queen => 'q',
                Piece::Rook => 'r',
                Piece::Bishop => 'b',
                Piece::Knight => 'n',
                _ => panic!("Invalid promotion piece"),
            };
            uci.push(promo_char);
        }
        uci
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GameStatus {
    InProgress,
    Check,
    Checkmate { winner: Color },
    Stalemate,
    DrawByFiftyMoveRule,
    DrawByInsufficientMaterial,
    DrawByRepetition,
}
