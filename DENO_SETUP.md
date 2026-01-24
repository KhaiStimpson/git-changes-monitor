# Deno Installation Note

## Issue: Deno Not Found in PATH

If you see "deno: command not found", Deno is installed but not in your system PATH.

## Solutions

### Option 1: Restart Terminal (Recommended)
Close and reopen your terminal/Git Bash. Deno should be available after a fresh session.

### Option 2: Check Deno Installation Location

Find where Deno was installed:
```bash
# On Windows (PowerShell)
where.exe deno

# On Windows (cmd)
where deno

# In Git Bash
find ~ -name "deno.exe" 2>/dev/null
```

### Option 3: Add to PATH Manually

Common Deno installation locations:
- **Windows**: `C:\Users\<username>\.deno\bin\`
- **Mac/Linux**: `~/.deno/bin/`

#### For PowerShell:
```powershell
$env:Path += ";$env:USERPROFILE\.deno\bin"
```

#### For Git Bash:
```bash
export PATH="$HOME/.deno/bin:$PATH"
```

### Option 4: Install Deno (if not already installed)

```bash
# Windows (PowerShell)
irm https://deno.land/install.ps1 | iex

# Mac/Linux
curl -fsSL https://deno.land/install.sh | sh
```

### Option 5: Use Full Path Temporarily

If you know where Deno is installed, you can run:
```bash
# Example (adjust path to your installation)
C:/Users/YourName/.deno/bin/deno.exe task start
```

## Verify Installation

Once Deno is in PATH:
```bash
deno --version
```

You should see output like:
```
deno 1.x.x (release, x86_64-pc-windows-msvc)
v8 12.x.x
typescript 5.x.x
```

## Running the Application

After Deno is available:
```bash
# Run the application
deno task start

# Or with full permissions explicitly
deno run --allow-read --allow-write --allow-run --allow-env src/main.tsx
```
