use crate::chess_engine::board::Board;
use crate::chess_engine::types::{Color, Piece, Square, Move};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastlingRights {
    pub fn new() -> Self {
        CastlingRights {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }

    pub fn none() -> Self {
        CastlingRights {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        }
    }

    pub fn can_castle(&self, color: Color, kingside: bool) -> bool {
        match (color, kingside) {
            (Color::White, true) => self.white_kingside,
            (Color::White, false) => self.white_queenside,
            (Color::Black, true) => self.black_kingside,
            (Color::Black, false) => self.black_queenside,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub board: Board,
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    pub position_history: Vec<u64>,
}

impl Position {
    pub fn new() -> Self {
        let mut position = Position {
            board: Board::initial_position(),
            side_to_move: Color::White,
            castling_rights: CastlingRights::new(),
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            position_history: Vec::new(),
        };

        let hash = position.compute_zobrist_hash();
        position.position_history.push(hash);
        position
    }

    pub fn empty() -> Self {
        Position {
            board: Board::new(),
            side_to_move: Color::White,
            castling_rights: CastlingRights::none(),
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            position_history: Vec::new(),
        }
    }

    pub fn compute_zobrist_hash(&self) -> u64 {
        let mut hash = 0u64;

        // Hash pieces
        for i in 0..64 {
            if let Some((piece, color)) = self.board.get(Square::new(i as u8).unwrap()) {
                let piece_index = match piece {
                    Piece::Pawn => 0,
                    Piece::Knight => 1,
                    Piece::Bishop => 2,
                    Piece::Rook => 3,
                    Piece::Queen => 4,
                    Piece::King => 5,
                };
                let color_index = match color {
                    Color::White => 0,
                    Color::Black => 1,
                };
                hash ^= ZOBRIST_PIECES[i][color_index][piece_index];
            }
        }

        // Hash castling rights
        if self.castling_rights.white_kingside {
            hash ^= ZOBRIST_CASTLING[0];
        }
        if self.castling_rights.white_queenside {
            hash ^= ZOBRIST_CASTLING[1];
        }
        if self.castling_rights.black_kingside {
            hash ^= ZOBRIST_CASTLING[2];
        }
        if self.castling_rights.black_queenside {
            hash ^= ZOBRIST_CASTLING[3];
        }

        // Hash en passant
        if let Some(ep_square) = self.en_passant_target {
            hash ^= ZOBRIST_EN_PASSANT[ep_square.file() as usize];
        }

        // Hash side to move
        if self.side_to_move == Color::Black {
            hash ^= *ZOBRIST_SIDE_TO_MOVE;
        }

        hash
    }

    pub fn is_repetition(&self) -> bool {
        if self.position_history.len() < 3 {
            return false;
        }

        let current_hash = self.position_history.last().unwrap();
        let mut count = 0;

        for hash in &self.position_history {
            if hash == current_hash {
                count += 1;
                if count >= 3 {
                    return true;
                }
            }
        }

        false
    }

    pub fn has_insufficient_material(&self) -> bool {
        let white_pieces = self.board.pieces_of_color(Color::White);
        let black_pieces = self.board.pieces_of_color(Color::Black);

        // K vs K
        if white_pieces.len() == 1 && black_pieces.len() == 1 {
            return true;
        }

        // K+B vs K or K+N vs K
        if white_pieces.len() == 1 && black_pieces.len() == 2 {
            if black_pieces.iter().any(|(_, p)| *p == Piece::Bishop || *p == Piece::Knight) {
                return true;
            }
        }

        if black_pieces.len() == 1 && white_pieces.len() == 2 {
            if white_pieces.iter().any(|(_, p)| *p == Piece::Bishop || *p == Piece::Knight) {
                return true;
            }
        }

        // K+B vs K+B with same color bishops
        if white_pieces.len() == 2 && black_pieces.len() == 2 {
            let white_has_bishop = white_pieces.iter().find(|(_, p)| *p == Piece::Bishop);
            let black_has_bishop = black_pieces.iter().find(|(_, p)| *p == Piece::Bishop);

            if let (Some((white_sq, _)), Some((black_sq, _))) = (white_has_bishop, black_has_bishop) {
                // Check if bishops are on same color squares
                let white_square_color = (white_sq.rank() + white_sq.file()) % 2;
                let black_square_color = (black_sq.rank() + black_sq.file()) % 2;
                if white_square_color == black_square_color {
                    return true;
                }
            }
        }

        false
    }

    pub fn update_castling_rights_after_move(&mut self, mv: &Move) {
        // If king moves, remove all castling rights for that color
        if let Some((Piece::King, color)) = self.board.get(mv.from) {
            match color {
                Color::White => {
                    self.castling_rights.white_kingside = false;
                    self.castling_rights.white_queenside = false;
                }
                Color::Black => {
                    self.castling_rights.black_kingside = false;
                    self.castling_rights.black_queenside = false;
                }
            }
        }

        // If rook moves from starting position, remove that castling right
        if let Some((Piece::Rook, color)) = self.board.get(mv.from) {
            match (color, mv.from.index()) {
                (Color::White, 0) => self.castling_rights.white_queenside = false,
                (Color::White, 7) => self.castling_rights.white_kingside = false,
                (Color::Black, 56) => self.castling_rights.black_queenside = false,
                (Color::Black, 63) => self.castling_rights.black_kingside = false,
                _ => {}
            }
        }

        // If a rook is captured on its starting square, remove that castling right
        match mv.to.index() {
            0 => {
                if matches!(self.board.get(mv.to), Some((Piece::Rook, Color::White))) {
                    self.castling_rights.white_queenside = false;
                }
            }
            7 => {
                if matches!(self.board.get(mv.to), Some((Piece::Rook, Color::White))) {
                    self.castling_rights.white_kingside = false;
                }
            }
            56 => {
                if matches!(self.board.get(mv.to), Some((Piece::Rook, Color::Black))) {
                    self.castling_rights.black_queenside = false;
                }
            }
            63 => {
                if matches!(self.board.get(mv.to), Some((Piece::Rook, Color::Black))) {
                    self.castling_rights.black_kingside = false;
                }
            }
            _ => {}
        }
    }
}

// Zobrist hashing tables
static ZOBRIST_PIECES: Lazy<[[[u64; 6]; 2]; 64]> = Lazy::new(|| {
    let mut rng = ZobristRng::new(123456789);
    let mut table = [[[0u64; 6]; 2]; 64];
    for square in 0..64 {
        for color in 0..2 {
            for piece in 0..6 {
                table[square][color][piece] = rng.next();
            }
        }
    }
    table
});

static ZOBRIST_CASTLING: Lazy<[u64; 4]> = Lazy::new(|| {
    let mut rng = ZobristRng::new(987654321);
    [rng.next(), rng.next(), rng.next(), rng.next()]
});

static ZOBRIST_EN_PASSANT: Lazy<[u64; 8]> = Lazy::new(|| {
    let mut rng = ZobristRng::new(456789123);
    [
        rng.next(), rng.next(), rng.next(), rng.next(),
        rng.next(), rng.next(), rng.next(), rng.next()
    ]
});

static ZOBRIST_SIDE_TO_MOVE: Lazy<u64> = Lazy::new(|| {
    let mut rng = ZobristRng::new(321654987);
    rng.next()
});

// Simple LCG for deterministic random numbers
struct ZobristRng {
    state: u64,
}

impl ZobristRng {
    fn new(seed: u64) -> Self {
        ZobristRng { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }
}
