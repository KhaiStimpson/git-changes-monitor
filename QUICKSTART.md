# Quick Start Guide

## Installation

Since you have Deno installed, you can run the application directly or install it globally.

### Option 1: Run Directly

```bash
deno task start
```

### Option 2: Install Globally

```bash
deno task install
```

This will create a `gfm` command you can use from anywhere.

## Usage Examples

### Monitor Current Directory
```bash
# If installed globally
gfm

# Or run directly
deno task start
```

### Monitor Specific Directory
```bash
gfm /path/to/repo
```

### Use Custom Config
```bash
gfm --config ~/my-config.json
```

### Single Snapshot (No Live Updates)
```bash
gfm --no-watch
```

## Configuration

On first run, a config file will be created at:
- Linux/Mac: `~/.config/git-file-monitor/gfm.json`
- Windows: `%USERPROFILE%\.config\git-file-monitor\gfm.json`

Edit this file to customize:
- Display options (toggle features on/off)
- UI settings (colors, debounce timing, preview lines)
- Keybindings (customize shortcuts)

## Keyboard Shortcuts

Default shortcuts:
- `↑/↓` or `j/k` - Navigate files
- `PgUp/PgDn` - Page up/down
- `p` - Toggle preview panel
- `r` - Manual refresh
- `?` - Show help menu
- `q` or `Esc` - Quit

## Testing the Application

Try these commands to see the TUI in action:

```bash
# 1. Make some changes to test.md
echo "New line" >> test.md

# 2. Create a new file
touch new-file.txt

# 3. Run the app
deno task start
```

The TUI will automatically update as you make changes!

## Troubleshooting

### Deno not found
Make sure Deno is in your PATH. You may need to:
- Restart your terminal
- Add Deno to PATH manually

### Permission Errors
The application requires these permissions:
- `--allow-read` - Read files and Git repo
- `--allow-write` - Write config file
- `--allow-run` - Execute Git commands
- `--allow-env` - Read environment variables

These are already configured in `deno.json` tasks.

### Not a Git Repository
Make sure you run the command from within a Git repository, or provide a path to one.

## Development

### Run with Auto-Reload
```bash
deno task dev
```

### Build Standalone Executable
```bash
deno task build
```

This creates a `gfm` executable in the project root.

## Features

✅ Real-time file monitoring (no polling)
✅ Staged vs unstaged changes
✅ Line change counts (+/- stats)
✅ Diff preview panel
✅ Branch information
✅ Last commit info
✅ Keyboard navigation
✅ Fully configurable
✅ Cross-platform (Windows, Mac, Linux)
