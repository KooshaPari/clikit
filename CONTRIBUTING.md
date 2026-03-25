# Contributing to clikit

Thank you for your interest in contributing to clikit!

## Development Setup

```bash
# Clone the repository
git clone https://github.com/KooshaPari/clikit
cd clikit

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build

# Test
cargo test

# Lint
cargo clippy
cargo fmt --check
```

## Architecture

clikit follows hexagonal architecture:

```
src/
├── domain/           # Pure domain (no external deps)
├── application/      # Use cases
├── adapters/         # Port implementations
│   ├── primary/     # Driving adapters
│   └── secondary/   # Driven adapters
└── infrastructure/   # Cross-cutting concerns
```

## Making Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Make your changes
4. Add tests
5. Ensure all tests pass: `cargo test`
6. Run linting: `cargo clippy -- -D warnings`
7. Commit using conventional commits
8. Push and create a PR

## Code Style

- Run `cargo fmt` before committing
- Follow Rust idioms
- Write self-documenting code
- Keep functions small (max 50 lines)
- Document public APIs with doc comments

## Testing

- Unit tests in `mod tests` blocks
- Integration tests in `tests/` directory
- Property-based tests for complex logic
- Aim for 80% coverage

## Documentation

- Update README.md for user-facing changes
- Add doc comments for public APIs
- Update CHANGELOG.md
- Create ADR for architecture changes
