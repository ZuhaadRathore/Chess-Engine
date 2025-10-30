# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned Features
- Enhanced move annotations and board highlights for local review (Phase CHESS-2)
- Time controls and clock management for pass-and-play (Phase CHESS-3)
- PGN import/export and session persistence (Phase CHESS-4)
- Engine-backed analysis tools and hinting (Phase CHESS-5)
- Accessibility and personalization upgrades (Phase CHESS-6)

## [0.1.0] - 2025-01-XX

### Added
- Initial release of the chess application
- Cross-platform support (Windows, macOS, Linux, Android, iOS)
- Full chess engine implementation in Rust
- Legal move generation and validation
- Checkmate, stalemate, and draw detection
- FEN import/export support
- Local pass-and-play multiplayer
- Automatic board rotation between turns
- Move history tracking with algebraic notation
- Captured pieces display
- Manual and system theme support (light/dark)
- Haptic feedback for mobile devices
- Touch-optimized UI with 48px+ touch targets
- Move analysis with position evaluation
- Best move hints
- Evaluation bar showing position advantage
- Evaluation graph tracking game progression
- Game summary with key statistics
- Comprehensive test suite (Vitest + Cargo)
- Mobile-friendly responsive design
- GPU-accelerated rendering

### Technical Details
- Built with Tauri v2 for cross-platform support
- Rust backend for chess engine and game logic
- React 18 + TypeScript frontend
- Vite build tool for fast development
- Vitest for frontend testing
- Cargo for Rust testing and building
- Context API for theme management
- Custom hooks for game state and analysis

[unreleased]: https://github.com/yourusername/chess-engine/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/chess-engine/releases/tag/v0.1.0
