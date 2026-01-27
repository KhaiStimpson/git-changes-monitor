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

### Windows

#### Option 1: PowerShell Installer (Recommended)

The easiest way to install on Windows is using the PowerShell installation script:

```powershell
irm https://raw.githubusercontent.com/KhaiStimpson/git-changes-monitor/main/install.ps1 | iex
```

**Security Note:** Review the script before running by visiting the [install.ps1](https://raw.githubusercontent.com/KhaiStimpson/git-changes-monitor/main/install.ps1) URL in your browser, or download and run locally:

```powershell
# Download and review the script first
Invoke-WebRequest -Uri https://raw.githubusercontent.com/KhaiStimpson/git-changes-monitor/main/install.ps1 -OutFile install.ps1
# Review the script, then run it
.\install.ps1
```

This will:
- Download the latest release
- Install to `%LOCALAPPDATA%\Programs\gfm`
- Add to your PATH automatically
- Enable easy updates by running the same command

#### Option 2: Scoop Package Manager

If you use [Scoop](https://scoop.sh/), you can install gfm with:

```powershell
scoop install https://raw.githubusercontent.com/KhaiStimpson/git-changes-monitor/main/gfm.json
```

To update:
```powershell
scoop update gfm
```

#### Option 3: Manual Installation

1. Download `gfm-windows-x86_64.exe` from the [Releases page](https://github.com/KhaiStimpson/git-changes-monitor/releases)
2. Rename it to `gfm.exe`
3. Move it to a directory in your PATH (e.g., `C:\Program Files\gfm\`)

### Linux

Pre-built binaries for Linux are available on the [Releases page](https://github.com/KhaiStimpson/git-changes-monitor/releases).

Download the appropriate binary:
- `gfm-linux-x86_64` - Linux 64-bit (dynamically linked)
- `gfm-linux-x86_64-musl` - Linux 64-bit (statically linked, works on most distributions)

```bash
# Download and install
curl -L https://github.com/KhaiStimpson/git-changes-monitor/releases/latest/download/gfm-linux-x86_64 -o gfm
chmod +x gfm
sudo mv gfm /usr/local/bin/
```

### From Source

**Prerequisites:**
- [Rust](https://rustup.rs/) (1.70 or later)
- Git installed and accessible in PATH

```bash
# Clone the repository
git clone https://github.com/KhaiStimpson/git-changes-monitor
cd git-changes-monitor

# Install globally
cargo install --path .

# Or just build
cargo build --release
```

The binary will be available at `target/release/gfm` (or `gfm.exe` on Windows).

### Future: Via Cargo

```bash
# Coming soon
cargo install gfm
```

## Updating

### Windows

To update gfm on Windows, simply run the installation command again:

**PowerShell Installer:**
```powershell
irm https://raw.githubusercontent.com/KhaiStimpson/git-changes-monitor/main/install.ps1 | iex
```

**Scoop:**
```powershell
scoop update gfm
```

### Linux

Download and replace the binary with the latest version:
```bash
curl -L https://github.com/KhaiStimpson/git-changes-monitor/releases/latest/download/gfm-linux-x86_64 -o gfm
chmod +x gfm
sudo mv gfm /usr/local/bin/
```

### From Source

```bash
cd git-changes-monitor
git pull
cargo install --path .
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

## Troubleshooting

### Windows

**PowerShell Execution Policy Error**

If you get an error about execution policies when running the installer:

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

Then run the installer again.

**"gfm is not recognized" after installation**

1. Close and reopen your terminal (PATH changes require a new terminal session)
2. Verify the installation directory is in your PATH:
   ```powershell
   $env:Path -split ';' | Select-String gfm
   ```
3. If not found, manually add it or run the installer again to update PATH

**Antivirus blocking the download**

Some antivirus software may flag the downloaded executable. This is a false positive. You can:
1. Add an exception for the installation directory
2. Download manually from GitHub Releases
3. Build from source

### Linux

**Permission denied when running gfm**

Make sure the binary is executable:
```bash
chmod +x gfm
```

**Command not found after installation**

Ensure `/usr/local/bin` is in your PATH:
```bash
echo $PATH | grep /usr/local/bin
```

If not, add it to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.):
```bash
export PATH="/usr/local/bin:$PATH"
```

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
