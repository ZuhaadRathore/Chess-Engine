use crate::chess_engine::types::{Color, Piece, Square};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    squares: [Option<(Piece, Color)>; 64],
}

impl Serialize for Board {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.squares.as_slice().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Board {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let squares_vec: Vec<Option<(Piece, Color)>> = Vec::deserialize(deserializer)?;
        if squares_vec.len() != 64 {
            return Err(serde::de::Error::custom(format!(
                "Expected 64 squares, got {}",
                squares_vec.len()
            )));
        }
        let mut squares = [None; 64];
        squares.copy_from_slice(&squares_vec);
        Ok(Board { squares })
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            squares: [None; 64],
        }
    }

    pub fn initial_position() -> Self {
        let mut board = Board::new();

        // White pieces
        board.set(Square::new(0).unwrap(), Some((Piece::Rook, Color::White)));
        board.set(Square::new(1).unwrap(), Some((Piece::Knight, Color::White)));
        board.set(Square::new(2).unwrap(), Some((Piece::Bishop, Color::White)));
        board.set(Square::new(3).unwrap(), Some((Piece::Queen, Color::White)));
        board.set(Square::new(4).unwrap(), Some((Piece::King, Color::White)));
        board.set(Square::new(5).unwrap(), Some((Piece::Bishop, Color::White)));
        board.set(Square::new(6).unwrap(), Some((Piece::Knight, Color::White)));
        board.set(Square::new(7).unwrap(), Some((Piece::Rook, Color::White)));

        // White pawns
        for i in 8..16 {
            board.set(Square::new(i).unwrap(), Some((Piece::Pawn, Color::White)));
        }

        // Black pawns
        for i in 48..56 {
            board.set(Square::new(i).unwrap(), Some((Piece::Pawn, Color::Black)));
        }

        // Black pieces
        board.set(Square::new(56).unwrap(), Some((Piece::Rook, Color::Black)));
        board.set(Square::new(57).unwrap(), Some((Piece::Knight, Color::Black)));
        board.set(Square::new(58).unwrap(), Some((Piece::Bishop, Color::Black)));
        board.set(Square::new(59).unwrap(), Some((Piece::Queen, Color::Black)));
        board.set(Square::new(60).unwrap(), Some((Piece::King, Color::Black)));
        board.set(Square::new(61).unwrap(), Some((Piece::Bishop, Color::Black)));
        board.set(Square::new(62).unwrap(), Some((Piece::Knight, Color::Black)));
        board.set(Square::new(63).unwrap(), Some((Piece::Rook, Color::Black)));

        board
    }

    pub fn get(&self, square: Square) -> Option<(Piece, Color)> {
        self.squares[square.index() as usize]
    }

    pub fn set(&mut self, square: Square, piece: Option<(Piece, Color)>) {
        self.squares[square.index() as usize] = piece;
    }

    pub fn is_empty(&self, square: Square) -> bool {
        self.squares[square.index() as usize].is_none()
    }

    pub fn find_king(&self, color: Color) -> Option<Square> {
        for i in 0..64 {
            if let Some((Piece::King, c)) = self.squares[i] {
                if c == color {
                    return Square::new(i as u8);
                }
            }
        }
        None
    }

    pub fn pieces_of_color(&self, color: Color) -> Vec<(Square, Piece)> {
        let mut pieces = Vec::new();
        for i in 0..64 {
            if let Some((piece, c)) = self.squares[i] {
                if c == color {
                    pieces.push((Square::new(i as u8).unwrap(), piece));
                }
            }
        }
        pieces
    }

    pub fn is_attacked_by(
        &self,
        square: Square,
        attacker_color: Color,
    ) -> bool {
        let target_rank = square.rank();
        let target_file = square.file();

        // Check for pawn attacks
        let pawn_direction = if attacker_color == Color::White { 1 } else { -1 };
        let pawn_rank = (target_rank as i8) - pawn_direction;

        if pawn_rank >= 0 && pawn_rank < 8 {
            for file_offset in [-1, 1] {
                let pawn_file = (target_file as i8) + file_offset;
                if pawn_file >= 0 && pawn_file < 8 {
                    if let Some(sq) = Square::from_rank_file(pawn_rank as u8, pawn_file as u8) {
                        if let Some((Piece::Pawn, color)) = self.get(sq) {
                            if color == attacker_color {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        // Check for knight attacks
        const KNIGHT_OFFSETS: [(i8, i8); 8] = [
            (-2, -1), (-2, 1), (-1, -2), (-1, 2),
            (1, -2), (1, 2), (2, -1), (2, 1),
        ];

        for (rank_offset, file_offset) in KNIGHT_OFFSETS {
            let knight_rank = (target_rank as i8) + rank_offset;
            let knight_file = (target_file as i8) + file_offset;

            if is_valid_square(knight_rank, knight_file) {
                if let Some(sq) = Square::from_rank_file(knight_rank as u8, knight_file as u8) {
                    if let Some((Piece::Knight, color)) = self.get(sq) {
                        if color == attacker_color {
                            return true;
                        }
                    }
                }
            }
        }

        // Check for king attacks
        const KING_OFFSETS: [(i8, i8); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        for (rank_offset, file_offset) in KING_OFFSETS {
            let king_rank = (target_rank as i8) + rank_offset;
            let king_file = (target_file as i8) + file_offset;

            if is_valid_square(king_rank, king_file) {
                if let Some(sq) = Square::from_rank_file(king_rank as u8, king_file as u8) {
                    if let Some((Piece::King, color)) = self.get(sq) {
                        if color == attacker_color {
                            return true;
                        }
                    }
                }
            }
        }

        // Check for sliding piece attacks (bishop, rook, queen)
        const BISHOP_DIRECTIONS: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        const ROOK_DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (rank_dir, file_dir) in BISHOP_DIRECTIONS {
            if self.is_attacked_along_ray(square, attacker_color, rank_dir, file_dir, &[Piece::Bishop, Piece::Queen]) {
                return true;
            }
        }

        for (rank_dir, file_dir) in ROOK_DIRECTIONS {
            if self.is_attacked_along_ray(square, attacker_color, rank_dir, file_dir, &[Piece::Rook, Piece::Queen]) {
                return true;
            }
        }

        false
    }

    fn is_attacked_along_ray(
        &self,
        square: Square,
        attacker_color: Color,
        rank_dir: i8,
        file_dir: i8,
        piece_types: &[Piece],
    ) -> bool {
        let mut rank = square.rank() as i8;
        let mut file = square.file() as i8;

        loop {
            rank += rank_dir;
            file += file_dir;

            if !is_valid_square(rank, file) {
                break;
            }

            if let Some(sq) = Square::from_rank_file(rank as u8, file as u8) {
                if let Some((piece, color)) = self.get(sq) {
                    if color == attacker_color && piece_types.contains(&piece) {
                        return true;
                    }
                    // Blocked by any piece
                    break;
                }
            }
        }

        false
    }
}

pub fn is_valid_square(rank: i8, file: i8) -> bool {
    rank >= 0 && rank < 8 && file >= 0 && file < 8
}
