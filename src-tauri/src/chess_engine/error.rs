use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChessError {
    #[error("Invalid FEN: {reason}")]
    InvalidFen { reason: String },

    #[error("Invalid move: {reason}")]
    InvalidMove { reason: String },

    #[error("Invalid square: {square}")]
    InvalidSquare { square: String },

    #[error("Game is over: {status}")]
    GameOver { status: String },

    #[error("Parse error: {input}")]
    ParseError { input: String },
}

pub type Result<T> = std::result::Result<T, ChessError>;
