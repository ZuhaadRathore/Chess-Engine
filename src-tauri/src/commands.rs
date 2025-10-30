use tauri::State;
use std::sync::Mutex;
use crate::chess_engine::{ChessGame, Position, Move, Square, GameStatus, Piece, MoveAnalysis, analyze_all_moves, Evaluator};

// State type for managing the chess game
pub type GameState = Mutex<ChessGame>;

/// Creates a new chess game, resetting to the starting position
#[tauri::command]
pub fn new_game(state: State<GameState>) -> Result<(), String> {
    let mut game = state.lock().map_err(|e| e.to_string())?;
    *game = ChessGame::new();
    Ok(())
}

/// Returns the current board state with full game information
#[tauri::command]
pub fn get_board_state(state: State<GameState>) -> Result<Position, String> {
    let game = state.lock().map_err(|e| e.to_string())?;
    Ok(game.get_board_state().clone())
}

/// Returns all legal moves in the current position
#[tauri::command]
pub fn get_legal_moves(state: State<GameState>) -> Result<Vec<Move>, String> {
    let game = state.lock().map_err(|e| e.to_string())?;
    Ok(game.get_legal_moves())
}

/// Returns legal moves for a specific square
#[tauri::command]
pub fn get_legal_moves_for_square(state: State<GameState>, square: String) -> Result<Vec<Move>, String> {
    let parsed_square = Square::from_algebraic(&square).map_err(|e| e.to_string())?;
    let game = state.lock().map_err(|e| e.to_string())?;
    Ok(game.get_legal_moves_for_square(parsed_square))
}

/// Makes a move on the board and returns the updated game status
#[tauri::command]
pub fn make_move(
    state: State<GameState>,
    from: String,
    to: String,
    promotion: Option<String>,
) -> Result<GameStatus, String> {
    let from_square = Square::from_algebraic(&from).map_err(|e| e.to_string())?;
    let to_square = Square::from_algebraic(&to).map_err(|e| e.to_string())?;

    let promotion_piece = match promotion.as_deref() {
        Some(p) => Some(parse_promotion(p)?),
        None => None,
    };

    let mut game = state.lock().map_err(|e| e.to_string())?;

    // Get all legal moves and find the matching one with correct flags
    let legal_moves = game.get_legal_moves();
    let mv = legal_moves
        .into_iter()
        .find(|m| {
            m.from == from_square
                && m.to == to_square
                && m.promotion == promotion_piece
        })
        .ok_or_else(|| {
            format!(
                "Illegal move: {} to {}{}",
                from,
                to,
                promotion.map(|p| format!(" (promotion: {})", p)).unwrap_or_default()
            )
        })?;

    game.make_move(mv).map_err(|e| e.to_string())?;
    Ok(game.get_status())
}

/// Undoes the last move and returns the updated game status
#[tauri::command]
pub fn undo_move(state: State<GameState>) -> Result<GameStatus, String> {
    let mut game = state.lock().map_err(|e| e.to_string())?;
    game.undo_move().map_err(|e| e.to_string())?;
    Ok(game.get_status())
}

/// Returns the current game status
#[tauri::command]
pub fn get_game_status(state: State<GameState>) -> Result<GameStatus, String> {
    let game = state.lock().map_err(|e| e.to_string())?;
    Ok(game.get_status())
}

/// Loads a position from FEN notation
#[tauri::command]
pub fn load_fen(state: State<GameState>, fen: String) -> Result<Position, String> {
    let new_game = ChessGame::from_fen(&fen).map_err(|e| e.to_string())?;
    let position = new_game.get_board_state().clone();

    let mut game = state.lock().map_err(|e| e.to_string())?;
    *game = new_game;
    Ok(position)
}

/// Returns the FEN string representation of the current position
#[tauri::command]
pub fn get_fen(state: State<GameState>) -> Result<String, String> {
    let game = state.lock().map_err(|e| e.to_string())?;
    Ok(game.to_fen())
}

/// Analyzes a specific move and returns detailed information
#[tauri::command]
pub fn analyze_move(
    state: State<GameState>,
    from: String,
    to: String,
    promotion: Option<String>,
) -> Result<MoveAnalysis, String> {
    let from_square = Square::from_algebraic(&from).map_err(|e| e.to_string())?;
    let to_square = Square::from_algebraic(&to).map_err(|e| e.to_string())?;

    let promotion_piece = match promotion.as_deref() {
        Some(p) => Some(parse_promotion(p)?),
        None => None,
    };

    let game = state.lock().map_err(|e| e.to_string())?;
    let position = game.get_board_state();

    // Find the matching move
    let legal_moves = game.get_legal_moves();
    let chess_move = legal_moves
        .into_iter()
        .find(|m| {
            m.from == from_square
                && m.to == to_square
                && m.promotion == promotion_piece
        })
        .ok_or_else(|| format!("Move not found: {} to {}", from, to))?;

    Ok(MoveAnalysis::analyze(&chess_move, position))
}

/// Analyzes all legal moves in the current position
#[tauri::command]
pub fn analyze_all_legal_moves(state: State<GameState>) -> Result<Vec<MoveAnalysis>, String> {
    let game = state.lock().map_err(|e| e.to_string())?;
    let position = game.get_board_state();
    Ok(analyze_all_moves(position))
}

/// Evaluates the current position and returns a score in centipawns
/// Positive = White advantage, Negative = Black advantage
#[tauri::command]
pub fn evaluate_position(state: State<GameState>) -> Result<i32, String> {
    let game = state.lock().map_err(|e| e.to_string())?;
    let position = game.get_board_state();
    Ok(Evaluator::evaluate(position))
}

/// Helper function to parse promotion string to Piece enum
/// Accepts case-insensitive input (e.g., "queen", "Queen", "QUEEN" all work)
fn parse_promotion(s: &str) -> Result<Piece, String> {
    match s.to_ascii_lowercase().as_str() {
        "queen" => Ok(Piece::Queen),
        "rook" => Ok(Piece::Rook),
        "bishop" => Ok(Piece::Bishop),
        "knight" => Ok(Piece::Knight),
        _ => Err(format!("Invalid promotion piece: {}. Must be Queen, Rook, Bishop, or Knight", s)),
    }
}
