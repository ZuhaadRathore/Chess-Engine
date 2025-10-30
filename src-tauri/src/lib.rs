mod commands;
mod chess_engine;

use std::sync::Mutex as StdMutex;
pub use chess_engine::ChessGame;

#[cfg(any(target_os = "android", target_os = "ios"))]
use tauri_plugin_haptics;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let game_state = StdMutex::new(ChessGame::new());

    let mut builder = tauri::Builder::default().manage(game_state);

    // Register shell plugin on desktop platforms only
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        builder = builder.plugin(tauri_plugin_shell::init());
    }

    // Register haptics plugin on mobile platforms
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        builder = builder.plugin(tauri_plugin_haptics::init());
    }

    builder
        .invoke_handler(tauri::generate_handler![
            // Chess commands
            commands::new_game,
            commands::get_board_state,
            commands::get_legal_moves,
            commands::get_legal_moves_for_square,
            commands::make_move,
            commands::undo_move,
            commands::get_game_status,
            commands::load_fen,
            commands::get_fen,
            // Analysis commands
            commands::analyze_move,
            commands::analyze_all_legal_moves,
            commands::evaluate_position,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
