# Chess Engine

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/react-%2320232a.svg?style=flat&logo=react&logoColor=%2361DAFB)](https://reactjs.org/)
[![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=flat&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Tauri](https://img.shields.io/badge/tauri-%2324C8DB.svg?style=flat&logo=tauri&logoColor=%23FFFFFF)](https://tauri.app/)

A cross-platform chess application built with Tauri v2, focused on polished local pass-and-play sessions. The app runs on Windows, macOS, Linux, Android, and iOS.

## Features

- **Cross-Platform**: Desktop (Windows, macOS, Linux) and Mobile (Android, iOS)
- **Local Multiplayer**: Pass-and-play experience with automatic board rotation and rich move history
- **Modern Tech Stack**: Rust backend for chess logic, React + TypeScript frontend
- **Mobile Optimizations**: Touch-friendly UI, haptic feedback, theme system, and performance tuning

## Mobile Features

The app is fully optimized for mobile devices with the following features:

### Theme System
- **Manual Theme Toggle**: Choose between light, dark, or system theme
- **System Preference Support**: Automatically adapts to device theme settings
- **Persistent Selection**: Theme preference is saved locally
- **Smooth Transitions**: Animated theme switching for a polished experience

### Touch Optimizations
- **Large Touch Targets**: All interactive elements are at least 48x48px for easy tapping
- **Haptic Feedback**: Tactile feedback for moves, captures, game events, and UI interactions
- **Touch-Friendly Board**: Optimized chess board interactions for one-handed use
- **Active States**: Visual feedback on touch for better responsiveness

### Performance
- **GPU-Accelerated Rendering**: Smooth 60fps animations on mobile devices
- **Memoized Components**: Efficient updates minimize re-renders
- **Optimized Asset Loading**: Fast initial load and smooth interactions
- **Mobile Viewport Handling**: Proper support for notched displays and safe areas

### Haptic Feedback Features
- **Move Feedback**: Subtle haptic response when making moves
- **Game Events**: Haptic notifications for checkmate, stalemate, and draws
- **UI Interactions**: Light haptic feedback for button presses and selections

## Tech Stack

- **Backend**: Rust (chess engine, game logic)
- **Frontend**: React 18 + TypeScript
- **Framework**: Tauri v2
- **Build Tool**: Vite
- **Styling**: CSS with CSS Variables for theming
- **Testing**: Vitest (unit/integration/e2e) + Cargo tests for engine validation

## Prerequisites

Before you begin, ensure you have the following installed:

- **Node.js** (v18 or later): https://nodejs.org/
- **Rust** (latest stable): https://rustup.rs/
- **Tauri CLI**: Will be installed with npm dependencies

### Additional Prerequisites for Mobile Development

- **Android**:
  - Android Studio
  - Android SDK (API level 24 or higher)
  - Android NDK
  - JDK 17 or later
  - See: https://v2.tauri.app/start/prerequisites/#android

- **iOS** (macOS only):
  - Xcode (14.0 or later)
  - Xcode Command Line Tools
  - See: https://v2.tauri.app/start/prerequisites/#ios

## Getting Started

### Installation

1. Clone the repository and navigate to the project directory

2. Install Node.js dependencies:
   ```bash
   npm install
   ```

3. The Rust dependencies will be installed automatically when you run the dev or build commands

### Development

#### Desktop Development

Run the desktop development server:
```bash
npm run tauri:dev
```

This will start both the Vite dev server and the Tauri application.

#### Mobile Development - Android

1. Initialize Android project (first time only):
   ```bash
   npm run tauri android init
   ```

2. Run on Android device/emulator:
   ```bash
   npm run tauri android dev
   ```

#### Mobile Development - iOS (macOS only)

1. Initialize iOS project (first time only):
   ```bash
   npm run tauri ios init
   ```

2. Run on iOS device/simulator:
   ```bash
   npm run tauri ios dev
   ```

### Setting Up App Icons and Splash Screens

The app requires icons for desktop, mobile, and web platforms. A comprehensive setup system is provided.

#### Quick Start

1. **Create a source icon**:
   - Create a 1024×1024 PNG image
   - Save it as `src-tauri/icons/app-icon.png`
   - Use a simple, high-contrast design (e.g., chess piece or board)

2. **Generate all icons**:
   ```bash
   # Windows (PowerShell)
   .\generate-icons.ps1

   # macOS/Linux (Bash)
   ./generate-icons.sh
   ```

This automatically generates:
- Desktop icons (.ico, .icns, .png)
- Mobile platform icons (Android/iOS)
- Web/PWA icons (192px, 512px)

#### Detailed Instructions

For comprehensive documentation including:
- Icon design guidelines
- Manual generation steps
- Platform-specific requirements
- Splash screen customization
- Troubleshooting tips

See the complete guide: **[src-tauri/icons/ICON_GENERATION_GUIDE.md](src-tauri/icons/ICON_GENERATION_GUIDE.md)**

#### Splash Screens

After initializing mobile platforms, customize splash screens:

- **Android**: Edit `src-tauri/gen/android/app/src/main/res/drawable/launch_background.xml` or `res/values/styles.xml`
- **iOS**: Open `src-tauri/gen/apple` in Xcode and edit `LaunchScreen.storyboard`

See the icon generation guide for detailed splash screen customization instructions.

### Building for Production

#### Desktop

Build desktop binaries:
```bash
npm run tauri:build
```

The built application will be in `src-tauri/target/release/bundle/`

#### Android

Build Android APK/AAB:
```bash
npm run tauri android build
```

#### iOS (macOS only)

Build iOS IPA:
```bash
npm run tauri ios build
```

### Mobile Testing Checklist

When testing on mobile devices, verify the following:

- **Haptic Feedback**: Test on physical devices (haptics don't work in simulators/emulators)
- **Theme Switching**: Verify theme toggle works and persists across app restarts
- **Touch Targets**: Ensure all buttons and interactive elements are easily tappable
- **Screen Sizes**: Test on various devices (phones, tablets) and orientations
- **Performance**: Check for smooth 60fps animations during gameplay
- **Safe Areas**: Verify proper layout on notched devices (iPhone X+)
- **Pass-and-Play Flow**: Confirm auto-flip, undo, and new game controls behave as expected

## Testing

### Frontend (Vitest)
```bash
pnpm test
```

### Engine (Rust)
```bash
cargo test
```

## Project Structure

```
Chess-Engine/
  src/                      # React frontend source
    components/             # ChessBoard UI, MoveHistory, ThemeToggle
    pages/                  # LocalGamePage (main UI)
    types/                  # TypeScript type definitions
    App.tsx                 # Main App component rendering the local experience
    main.tsx                # React entry point
    App.css                 # App styles
    index.css               # Global styles
  src-tauri/                # Rust backend source
    src/
      main.rs               # Desktop entry point
      lib.rs                # Shared library (mobile entry point)
    Cargo.toml              # Rust dependencies
    tauri.conf.json         # Tauri base configuration
    tauri.android.conf.json # Android-specific config
    tauri.ios.conf.json     # iOS-specific config
  index.html                # HTML entry point
  vite.config.ts            # Vite configuration
  tsconfig.json             # TypeScript configuration
  package.json              # Node.js dependencies and scripts
```


## Chess Engine API

The application provides the following Tauri commands for interacting with the chess engine. All commands are available via the TypeScript wrappers in `src/types/tauri.ts`.

### Core Game Commands

#### `new_game()`
Creates a new chess game, resetting to the starting position.

- **Parameters**: None
- **Returns**: `Promise<void>`
- **Example**:
  ```typescript
  await newGame();
  ```

#### `get_board_state()`
Returns the current board state with full game information.

- **Parameters**: None
- **Returns**: `Promise<Position>` - Complete position including board array (64 squares), side to move, castling rights, en passant target, halfmove clock, fullmove number, and position history
- **Example**:
  ```typescript
  const position = await getBoardState();
  console.log(position.side_to_move); // "White" or "Black"
  ```

#### `get_game_status()`
Returns the current game status.

- **Parameters**: None
- **Returns**: `Promise<GameStatus>` - Status object with type: InProgress, Check, Checkmate, Stalemate, DrawByFiftyMoveRule, DrawByInsufficientMaterial, or DrawByRepetition
- **Example**:
  ```typescript
  const status = await getGameStatus();
  if (status.type === 'Checkmate') {
    console.log(`${status.winner} wins!`);
  }
  ```

### Move Commands

#### `get_legal_moves()`
Returns all legal moves in the current position.

- **Parameters**: None
- **Returns**: `Promise<Move[]>` - Array of all legal moves with from/to squares, promotion piece, and move flags
- **Example**:
  ```typescript
  const moves = await getLegalMoves();
  console.log(`${moves.length} legal moves available`);
  ```

#### `get_legal_moves_for_square(square: string)`
Returns legal moves for a specific square.

- **Parameters**:
  - `square: string` - Algebraic square notation (e.g., "e4", "a1", "h8")
- **Returns**: `Promise<Move[]>` - Array of legal moves for the piece on that square
- **Example**:
  ```typescript
  const moves = await getLegalMovesForSquare("e2");
  // Returns moves like [{from: {index: 12}, to: {index: 20}, ...}, ...]
  ```

#### `make_move(from: string, to: string, promotion?: string)`
Makes a move on the board.

- **Parameters**:
  - `from: string` - Origin square in algebraic notation (e.g., "e2")
  - `to: string` - Destination square in algebraic notation (e.g., "e4")
  - `promotion?: 'Queen' | 'Rook' | 'Bishop' | 'Knight'` - Optional promotion piece for pawn promotions
- **Returns**: `Promise<GameStatus>` - Updated game status after the move
- **Throws**: Error if move is illegal, notation is invalid, or command fails
- **Example**:
  ```typescript
  const status = await makeMove("e2", "e4");
  // For pawn promotion:
  const status = await makeMove("e7", "e8", "Queen");
  ```

#### `undo_move()`
Undoes the last move.

- **Parameters**: None
- **Returns**: `Promise<GameStatus>` - Game status after undo
- **Throws**: Error if no moves to undo
- **Example**:
  ```typescript
  const status = await undoMove();
  ```

### FEN Commands

#### `load_fen(fen: string)`
Loads a position from FEN (Forsyth-Edwards Notation).

- **Parameters**:
  - `fen: string` - FEN string representing the position
- **Returns**: `Promise<Position>` - The loaded position
- **Throws**: Error if FEN is invalid
- **Example**:
  ```typescript
  const position = await loadFen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
  ```

#### `get_fen()`
Returns the FEN string representation of the current position.

- **Parameters**: None
- **Returns**: `Promise<string>` - FEN string of current position
- **Example**:
  ```typescript
  const fen = await getFen();
  console.log(fen); // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
  ```

### Type Definitions

All type definitions are available in `src/types/index.ts`:
- `Position`: Complete game state
- `Move`: Chess move with from/to squares and flags
- `GameStatus`: Discriminated union of game states
- `Color`: "White" | "Black"
- `Piece`: "Pawn" | "Knight" | "Bishop" | "Rook" | "Queen" | "King"

**Note on Square Notation**: All square parameters use algebraic notation where files are letters a-h (left to right) and ranks are numbers 1-8 (bottom to top from White's perspective). Examples: "a1" (bottom-left), "h8" (top-right), "e4" (center).

## Planned Features

The following features will be implemented in upcoming phases:

- **Phase CHESS-2**: Enhanced move annotations and board highlights for local review
- **Phase CHESS-3**: Time controls and clock management for pass-and-play
- **Phase CHESS-4**: PGN import/export and session persistence
- **Phase CHESS-5**: Engine-backed analysis tools and hinting
- **Phase CHESS-6**: Accessibility and personalization upgrades (high-contrast themes, larger pieces)


## Development Notes

- Tauri v2 uses a unified codebase for desktop and mobile via `lib.rs`
- Mobile-specific configurations are in separate JSON files that override the base config
- The dev server is configured to work with mobile devices via `TAURI_DEV_HOST` environment variable
- Vitest covers TypeScript unit/integration/e2e tests while `cargo test` validates the engine


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please read the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines on how to contribute to this project.

## Author

Created as a portfolio project demonstrating:
- Cross-platform desktop and mobile development with Tauri v2
- Chess engine implementation in Rust
- Modern React + TypeScript frontend architecture
- Mobile-first UI/UX design with haptic feedback
- Comprehensive testing with Vitest and Cargo
