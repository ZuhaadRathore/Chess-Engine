# Contributing to Chess Engine

Thank you for your interest in contributing to this chess application! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Process](#development-process)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Commit Message Guidelines](#commit-message-guidelines)

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

1. **Fork the repository** and clone your fork locally
2. **Install dependencies**:
   ```bash
   pnpm install
   ```
3. **Set up the development environment** following the instructions in [README.md](README.md)
4. **Create a new branch** for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Process

### Running the Development Server

For desktop development:
```bash
pnpm tauri:dev
```

For mobile development (Android/iOS), see the README for platform-specific instructions.

### Running Tests

Run frontend tests:
```bash
pnpm test
```

Run backend tests:
```bash
cd src-tauri
cargo test
```

### Building the Project

Test production builds:
```bash
pnpm tauri:build
```

## Pull Request Process

1. **Ensure your code follows the coding standards** (see below)
2. **Write or update tests** for your changes
3. **Update documentation** if you're changing functionality
4. **Run all tests** and ensure they pass
5. **Update the README.md** if needed with details of changes to the interface
6. **Commit your changes** following the commit message guidelines
7. **Push to your fork** and submit a pull request

### Pull Request Guidelines

- Provide a clear description of the problem and solution
- Include any relevant issue numbers
- Add screenshots or GIFs for UI changes
- Ensure CI checks pass
- Request review from maintainers

## Coding Standards

### TypeScript/React

- Use TypeScript for all new frontend code
- Follow the existing code style (use ESLint/Prettier if configured)
- Use functional components with hooks
- Keep components small and focused
- Use meaningful variable and function names
- Add JSDoc comments for complex functions

### Rust

- Follow standard Rust conventions and idioms
- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common mistakes
- Add documentation comments (`///`) for public APIs
- Prefer `Result` and `Option` over panicking
- Keep functions small and focused

### File Organization

- Frontend components go in `src/components/`
- Utility functions go in `src/utils/`
- Type definitions go in `src/types/`
- Rust modules are organized in `src-tauri/src/chess_engine/`

## Testing Guidelines

### Frontend Tests (Vitest)

- Write unit tests for utility functions
- Write integration tests for React components
- Write e2e tests for critical user flows
- Use React Testing Library best practices
- Mock Tauri commands in tests

Example test location: `src/utils/__tests__/chess.test.ts`

### Backend Tests (Cargo)

- Write unit tests for chess logic
- Test edge cases and error conditions
- Use property-based testing where appropriate
- Ensure 100% coverage for critical paths (move generation, validation)

Example test location: `src-tauri/src/chess_engine/tests.rs`

## Commit Message Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation changes
- **style**: Code style changes (formatting, no code change)
- **refactor**: Code refactoring
- **test**: Adding or updating tests
- **chore**: Maintenance tasks

### Examples

```
feat(board): add piece drag and drop functionality

Implement drag and drop for moving pieces on the chess board.
Includes touch support for mobile devices.

Closes #42
```

```
fix(engine): correct en passant validation logic

En passant moves were incorrectly validated when the capturing
pawn was on certain squares. This fixes the validation to check
the correct conditions.
```

## Architecture Overview

### Frontend Architecture

- **React**: UI framework
- **TypeScript**: Type safety
- **Vite**: Build tool and dev server
- **Tauri API**: Bridge to Rust backend
- **Context API**: Global state (theme)
- **Custom Hooks**: Reusable logic (game state, analysis)

### Backend Architecture

- **Chess Engine**: Pure Rust implementation
- **Move Generation**: Bitboard-based or array-based (check implementation)
- **Game State**: Immutable position tracking
- **FEN Support**: Standard chess notation
- **Tauri Commands**: IPC bridge to frontend

## Getting Help

- Check existing [issues](../../issues) and [pull requests](../../pulls)
- Open a new issue for bugs or feature requests
- Ask questions in issue comments
- Reach out to maintainers for guidance

## Recognition

Contributors will be recognized in the project. Thank you for helping make this chess application better!
