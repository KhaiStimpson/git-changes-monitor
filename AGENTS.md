# AGENTS.md - Git File Monitor (gfm)

This document provides guidance for AI coding agents working in this repository.

## Project Overview

**gfm** (Git File Monitor) is a Rust TUI application for monitoring Git repository changes in real-time. Built with:
- **ratatui** - TUI framework
- **crossterm** - Cross-platform terminal manipulation  
- **tokio** - Async runtime
- **color-eyre** - Error handling

## Build Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build

# Run
cargo run                      # Run in dev mode
cargo run -- --no-watch        # Run with arguments
cargo run -- /path/to/repo     # Monitor specific repository

# Install
cargo install --path .         # Install globally as 'gfm'
```

## Lint and Format

```bash
# Lint with Clippy
cargo clippy                   # Run linter
cargo clippy --fix             # Auto-fix warnings

# Format
cargo fmt                      # Format all code
cargo fmt -- --check           # Check formatting without changes
```

## Testing

```bash
# Run all tests
cargo test

# Run a single test by name
cargo test test_sanitize_removes_emojis

# Run tests in a specific module
cargo test ui::utils::tests

# Run tests with output
cargo test -- --nocapture

# Run tests matching a pattern
cargo test sanitize
```

Tests are located inline in source files using `#[cfg(test)]` modules. See `src/ui/utils.rs` for examples.

## Project Structure

```
src/
├── main.rs           # CLI entry point (clap argument parsing)
├── app.rs            # Application state and main event loop
├── event.rs          # Event handling (keyboard, file changes)
├── tui.rs            # Terminal initialization/cleanup
├── config/           # Configuration (types.rs, loader.rs)
├── git/              # Git operations (types.rs, service.rs)
├── theme/            # Color themes (themes.rs)
├── ui/               # UI components (render.rs, file_list.rs, file_preview.rs, etc.)
└── watcher/          # File system watching (service.rs)
```

## Code Style Guidelines

### Import Organization

Order imports separated by blank lines: std library, external crates, `crate::` imports, `super::` imports.

```rust
use std::path::PathBuf;

use color_eyre::eyre::Result;
use tokio::sync::mpsc;

use crate::config::types::Config;

use super::types::FileStatus;
```

### Naming Conventions

| Element | Convention | Example |
|---------|------------|---------|
| Files/Modules | `snake_case` | `file_list.rs`, `branch_info.rs` |
| Types/Structs/Enums | `PascalCase` | `FileStatus`, `GitService` |
| Functions/Methods | `snake_case` | `get_status`, `handle_key_event` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_BUFFER_SIZE` |
| Enum Variants | `PascalCase` | `FileStatusType::Modified` |

### Error Handling

Use `color_eyre` for error handling:

```rust
use color_eyre::eyre::{eyre, Result};

pub async fn get_status(&self) -> Result<GitStatus> {
    let output = Command::new("git").args(["status"]).output().await?;
    
    if !output.status.success() {
        return Err(eyre!("Failed to get status"));
    }
    Ok(status)
}
```

### Struct Patterns

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub display: DisplayConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self { display: DisplayConfig::default() }
    }
}

impl GitService {
    pub fn new(repo_path: PathBuf) -> Self {
        Self { repo_path }
    }
}
```

- Each feature is a subdirectory with `mod.rs`
- `mod.rs` contains module declarations and `pub use` re-exports
- Types go in `types.rs`, logic in `service.rs`
- UI components split by widget/responsibility

```rust
// In mod.rs
mod service;
mod types;

pub use service::GitService;
pub use types::{FileStatus, GitStatus};
```

### Testing

Write inline tests using `#[cfg(test)]` modules:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        assert_eq!(actual, expected);
    }
}
```

### Things to Avoid

- Don't use `unwrap()` or `expect()` without good reason - prefer `?` or `unwrap_or_default()`
- Don't block async code with synchronous operations
- Don't use `println!` in TUI code - it interferes with terminal rendering
- Don't forget to handle terminal cleanup on errors (see `tui::restore()`)
