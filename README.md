# Git Changes Monitor TUI (gfm)

A real-time terminal user interface for monitoring Git repository changes without polling.

## Features

- 🔄 **Real-time monitoring** - Uses `Deno.watchFs` for instant file change detection
- 📊 **Comprehensive status** - Shows staged/unstaged files, line changes, branch info
- 🎨 **Interactive TUI** - Built with OpenTUI for a smooth terminal experience
- ⚙️ **Highly configurable** - Customize display options and keybindings
- 🚀 **Fast & lightweight** - Native Deno performance

## Installation

### Prerequisites

- [Deno](https://deno.land/) installed
- Git installed and accessible in PATH

### Install from source

```bash
# Clone the repository
git clone <repo-url>
cd git-changes-monitor-tui

# Install globally
deno task install

# Or run directly
deno task start
```

## Usage

```bash
# Monitor current directory
gfm

# Monitor specific directory
gfm /path/to/repo

# With custom config
gfm --config ~/.custom-gfm.json

# Single snapshot (no live updates)
gfm --no-watch

# Show help
gfm --help
```

## Configuration

Configuration file location: `~/.config/git-file-monitor/gfm.json`

The config file is auto-created with defaults on first run. You can customize:

- **Display options**: Toggle visibility of file paths, line counts, staged vs unstaged, preview, branch info
- **UI settings**: Color scheme, debounce timing, preview line limits
- **Keybindings**: Customize keyboard shortcuts

See `config/default-config.json` for the full configuration schema.

## Keyboard Shortcuts

Default keybindings:

- `↑/↓` or `j/k` - Navigate file list
- `Enter` - Select/deselect file
- `p` - Toggle file preview panel
- `r` - Manual refresh
- `?` - Show help menu
- `q` or `Esc` - Quit

## Development

```bash
# Run in development mode (with auto-reload)
deno task dev

# Build standalone executable
deno task build
```

## Permissions

The application requires the following Deno permissions:

- `--allow-read`: Read files, Git repository, and config
- `--allow-write`: Write configuration file
- `--allow-run`: Execute Git commands
- `--allow-env`: Read environment variables (HOME, USERPROFILE)

## License

MIT
