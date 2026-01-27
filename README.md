# Git File Monitor (gfm)

A blazingly fast, real-time terminal user interface for monitoring Git repository changes built with Rust and Ratatui.

## Features

- 🔄 **Real-time monitoring** - Watch file changes with efficient file system notifications
- 📊 **Comprehensive Git status** - View staged/unstaged files, branch info, and last commit
- 🎨 **Beautiful TUI** - Built with Ratatui for a smooth terminal experience
- ⚡ **Async performance** - Powered by Tokio for responsive operations
- 🎭 **4 color themes** - Catppuccin, Nord, Tokyo Night, and Dracula
- ⚙️ **Highly configurable** - Customize display, keybindings, and themes via JSON
- 🚀 **Cross-platform** - Works on Windows, macOS, and Linux

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (1.70 or later)
- Git installed and accessible in PATH

### From Source

```bash
# Clone the repository
git clone <repo-url>
cd git-changes-monitor-tui

# Install globally
cargo install --path .

# Or just build
cargo build --release
```

The binary will be available at `target/release/gfm` (or `gfm.exe` on Windows).

### From GitHub Releases

Pre-built binaries for Windows and Linux are available on the [Releases page](https://github.com/KhaiStimpson/git-changes-monitor/releases).

Download the appropriate binary for your platform:
- `gfm-windows-x86_64.exe` - Windows 64-bit
- `gfm-linux-x86_64` - Linux 64-bit (dynamically linked)
- `gfm-linux-x86_64-musl` - Linux 64-bit (statically linked, works on most distributions)

Make the binary executable (Linux/macOS only):
```bash
chmod +x gfm-linux-x86_64
```

Then move it to a directory in your PATH:
```bash
# Linux/macOS
sudo mv gfm-linux-x86_64 /usr/local/bin/gfm

# Windows: Move gfm-windows-x86_64.exe to a directory in your PATH
```

### Future: Via Cargo

```bash
# Coming soon
cargo install gfm
```

## Usage

```bash
# Monitor current directory
gfm

# Monitor specific directory
gfm /path/to/repo

# Use custom config file
gfm --config ~/.custom-gfm.json

# Single snapshot (no live updates)
gfm --no-watch

# Show help
gfm --help
```

## Configuration

Configuration file location: `~/.config/git-file-monitor/gfm.json`

The config file is auto-created with defaults on first run. You can customize:

- **Display options**: Toggle file paths, line counts, preview panel, branch info
- **UI settings**: Color scheme, refresh debounce timing, preview line limits
- **Keybindings**: Customize keyboard shortcuts

See `config/default-config.json` for the full configuration schema.

### Example Configuration

```json
{
  "ui": {
    "color_scheme": "catppuccin",
    "refresh_debounce_ms": 300
  },
  "keybindings": {
    "quit": "q",
    "refresh": "r",
    "toggle_preview": "p",
    "help": "?"
  }
}
```

## Keyboard Shortcuts

Default keybindings (customizable via config):

- `↑/↓` or `j/k` - Navigate file list
- `Tab` or `s` - Switch between staged/unstaged sections
- `PageUp/PageDown` - Page up/down in file list
- `p` - Toggle file preview panel
- `r` - Manual refresh
- `?` - Show help menu
- `q` or `Esc` - Quit application
- `Ctrl+C` - Force quit

## Themes

Choose from 4 beautiful color themes by setting `color_scheme` in your config:

- **catppuccin** (default) - Soothing pastel theme
- **nord** - Arctic, north-bluish color palette
- **tokyo-night** - A dark theme inspired by Tokyo's night
- **dracula** - Dark theme with carefully chosen colors

## Development

```bash
# Run in development mode
cargo run

# Run with arguments
cargo run -- --no-watch

# Build debug binary
cargo build

# Build optimized release binary
cargo build --release

# Run linter
cargo clippy

# Format code
cargo fmt

# Run tests
cargo test
```

## Project Structure

```
src/
├── main.rs              # CLI entry point with clap
├── app.rs               # Application state and main event loop
├── event.rs             # Event handling system
├── tui.rs               # Terminal initialization and cleanup
├── config/              # Configuration loading and types
│   ├── mod.rs
│   ├── types.rs         # Config structs
│   └── loader.rs        # Config file I/O
├── git/                 # Git operations
│   ├── mod.rs
│   ├── types.rs         # Git data structures
│   └── service.rs       # Git command execution
├── theme/               # Color themes
│   ├── mod.rs
│   └── themes.rs        # Theme definitions
├── ui/                  # UI rendering components
│   ├── mod.rs
│   ├── render.rs        # Main render function
│   ├── branch_info.rs   # Branch info widget
│   ├── file_list.rs     # File list widget
│   ├── file_preview.rs  # Diff preview widget
│   ├── status_bar.rs    # Status bar widget
│   └── help_menu.rs     # Help modal widget
└── watcher/             # File system watching
    ├── mod.rs
    └── service.rs       # File watcher implementation
```

## Technology Stack

- **Language**: Rust 🦀
- **TUI Framework**: [Ratatui](https://ratatui.rs/) - Terminal UI library
- **Terminal Backend**: [Crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- **Async Runtime**: [Tokio](https://tokio.rs/) - Asynchronous runtime
- **CLI Parsing**: [Clap](https://github.com/clap-rs/clap) - Command line argument parser
- **File Watching**: [Notify](https://github.com/notify-rs/notify) - Cross-platform file system notifications
- **Serialization**: [Serde](https://serde.rs/) - Serialization framework

## How It Works

1. **Initialization**: Loads configuration, sets up terminal, verifies Git repository
2. **Event Loop**: Spawns async tasks for:
   - Terminal events (keyboard input)
   - File system watching (notify crate)
   - Git status polling (on-demand)
3. **Rendering**: Uses Ratatui to draw UI components with selected theme
4. **Git Operations**: Shells out to `git` CLI for status, diff, and branch info
5. **State Management**: App state updated on events, triggers re-render

## Performance

- **Fast startup**: Minimal overhead, starts in milliseconds
- **Efficient watching**: Uses OS-native file system notifications (inotify, FSEvents, ReadDirectoryChangesW)
- **Async operations**: Non-blocking Git commands and UI rendering
- **Debounced updates**: Configurable debounce prevents excessive refreshes

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

### Release Process (for maintainers)

The project uses GitHub Actions to automatically build and release binaries for Windows and Linux.

To create a new release:

1. Update the version in `Cargo.toml`
2. Commit the version change
3. Create and push a new tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
4. GitHub Actions will automatically:
   - Build binaries for Windows and Linux (including musl variant)
   - Create a GitHub Release with the binaries attached
   - Generate release notes from commits

You can also manually trigger the workflow from the Actions tab if needed.

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Built with [Ratatui](https://ratatui.rs/), an amazing Rust TUI library
- Inspired by lazygit and other Git TUI tools
- Color themes adapted from popular terminal themes
