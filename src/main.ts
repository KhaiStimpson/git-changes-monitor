import { Command } from "@cliffy/command";
import { Table } from "@cliffy/table";
import { colors } from "@cliffy/ansi/colors";
import { GitService } from "./services/git.ts";
import { FileWatcherService } from "./services/watcher.ts";
import { ConfigService } from "./services/config.ts";
import type { Config } from "./types/config.ts";
import type { GitStatus, FileStatus } from "./types/git.ts";

// Terminal control codes
const CLEAR_SCREEN = "\x1b[2J";
const MOVE_CURSOR_HOME = "\x1b[H";
const HIDE_CURSOR = "\x1b[?25l";
const SHOW_CURSOR = "\x1b[?25h";

class GitMonitorApp {
  private gitService: GitService;
  private watcherService: FileWatcherService | null = null;
  private config: Config;
  private selectedIndex = 0;
  private status: GitStatus | null = null;
  private running = true;
  private showHelp = false;

  constructor(
    private repoPath: string,
    private configPath: string,
    private watchMode: boolean,
  ) {
    this.gitService = new GitService(repoPath);
    const configService = new ConfigService(configPath);
    this.config = configService.get();
  }

  async init() {
    // Load config
    const configService = new ConfigService(this.configPath);
    await configService.load(this.configPath);
    this.config = configService.get();
  }

  async start() {
    // Initialize config
    await this.init();

    // Hide cursor
    Deno.stdout.writeSync(new TextEncoder().encode(HIDE_CURSOR));

    // Set raw mode for keyboard input
    Deno.stdin.setRaw(true);

    // Start keyboard listener
    this.startKeyboardListener();

    // Initial render
    await this.refresh();

    // Start file watcher if in watch mode
    if (this.watchMode) {
      this.watcherService = new FileWatcherService(this.repoPath);
      this.watcherService.watch(
        () => this.refresh(),
        this.config.ui.refreshDebounceMs,
      );
    }
  }

  async refresh() {
    this.status = await this.gitService.getStatus();
    this.render();
  }

  private getAllFiles(): FileStatus[] {
    if (!this.status) return [];
    return [...this.status.stagedFiles, ...this.status.unstagedFiles];
  }

  private startKeyboardListener() {
    const buffer = new Uint8Array(8);
    
    const readKeys = async () => {
      while (this.running) {
        try {
          const n = await Deno.stdin.read(buffer);
          if (n === null) break;

          const key = new TextDecoder().decode(buffer.subarray(0, n));
          await this.handleKeyPress(key);
        } catch {
          break;
        }
      }
    };

    readKeys();
  }

  private async handleKeyPress(key: string) {
    // Help menu handling
    if (this.showHelp) {
      if (key === "?" || key === "\x1b") { // ? or Escape
        this.showHelp = false;
        this.render();
      }
      return;
    }

    const allFiles = this.getAllFiles();

    switch (key) {
      case "q": // Quit
      case "\x03": // Ctrl+C
      case "\x1b": // Escape
        this.stop();
        break;
      
      case "r": // Refresh
        await this.refresh();
        break;
      
      case "?": // Help
        this.showHelp = true;
        this.render();
        break;
      
      case "j": // Down (vim)
      case "\x1b[B": // Arrow down
        if (this.selectedIndex < allFiles.length - 1) {
          this.selectedIndex++;
          this.render();
        }
        break;
      
      case "k": // Up (vim)
      case "\x1b[A": // Arrow up
        if (this.selectedIndex > 0) {
          this.selectedIndex--;
          this.render();
        }
        break;
      
      case "\x1b[6~": // Page down
        this.selectedIndex = Math.min(
          this.selectedIndex + 10,
          allFiles.length - 1,
        );
        this.render();
        break;
      
      case "\x1b[5~": // Page up
        this.selectedIndex = Math.max(this.selectedIndex - 10, 0);
        this.render();
        break;
    }
  }

  private render() {
    if (!this.status) return;

    // Clear screen and move cursor to home
    Deno.stdout.writeSync(
      new TextEncoder().encode(CLEAR_SCREEN + MOVE_CURSOR_HOME),
    );

    const lines: string[] = [];

    // Show help menu overlay
    if (this.showHelp) {
      this.renderHelpMenu(lines);
      Deno.stdout.writeSync(new TextEncoder().encode(lines.join("\n") + "\n"));
      return;
    }

    // Branch info
    if (this.config.display.showBranchInfo) {
      lines.push(colors.bold.cyan("═".repeat(60)));
      
      let branchLine = colors.bold.white("Branch: ") + colors.green(this.status.branch.name);
      
      // Add tracking info if available
      if (this.status.branch.upstream) {
        branchLine += colors.dim(` → ${this.status.branch.upstream}`);
      }
      if (this.status.branch.ahead > 0 || this.status.branch.behind > 0) {
        const tracking = [];
        if (this.status.branch.ahead > 0) tracking.push(colors.green(`↑${this.status.branch.ahead}`));
        if (this.status.branch.behind > 0) tracking.push(colors.red(`↓${this.status.branch.behind}`));
        branchLine += " " + tracking.join(" ");
      }
      
      lines.push(branchLine);
      
      if (this.config.display.showLastCommitInfo && this.status.lastCommit) {
        const shortHash = this.status.lastCommit.hash;
        lines.push(
          colors.bold.white("Last Commit: ") +
            colors.yellow(shortHash) +
            " " +
            this.status.lastCommit.subject,
        );
      }
      lines.push(colors.bold.cyan("═".repeat(60)));
      lines.push("");
    }

    // File lists with table
    const allFiles = this.getAllFiles();
    
    if (allFiles.length === 0) {
      lines.push(colors.yellow("No changes detected"));
    } else {
      // Staged files
      if (
        this.config.display.showStagedVsUnstaged &&
        this.status.stagedFiles.length > 0
      ) {
        lines.push(colors.bold.green("Staged Files:"));
        this.renderFileTable(this.status.stagedFiles, 0, lines);
        lines.push("");
      }

      // Unstaged files
      if (
        this.config.display.showStagedVsUnstaged &&
        this.status.unstagedFiles.length > 0
      ) {
        const offset = this.status.stagedFiles.length;
        lines.push(colors.bold.yellow("Unstaged Files:"));
        this.renderFileTable(this.status.unstagedFiles, offset, lines);
        lines.push("");
      }
    }

    // Status bar
    lines.push(colors.bold.cyan("─".repeat(60)));
    const statusParts = [
      `[${colors.bold.white(this.config.keybindings.quit)}] Quit`,
      `[${colors.bold.white(this.config.keybindings.refresh)}] Refresh`,
      `[${colors.bold.white(this.config.keybindings.help)}] Help`,
    ];
    
    if (this.watchMode) {
      statusParts.push(colors.green("● Watching"));
    }
    
    lines.push(statusParts.join("  "));

    // Output all lines
    Deno.stdout.writeSync(new TextEncoder().encode(lines.join("\n") + "\n"));
  }

  private renderFileTable(
    files: FileStatus[],
    offset: number,
    lines: string[],
  ) {
    const table = new Table()
      .header(["", "Status", "File", "Changes"])
      .body(
        files.map((file, index) => {
          const globalIndex = offset + index;
          const isSelected = globalIndex === this.selectedIndex;
          const marker = isSelected ? "▶" : " ";
          
          const statusColor = file.staged ? colors.green : colors.yellow;
          const statusText = this.getStatusSymbol(file);
          
          const fileName = isSelected
            ? colors.bold.white(file.path)
            : file.path;
          
          const changes = this.config.display.showLineChangeCounts && file.linesAdded !== undefined
            ? colors.green(`+${file.linesAdded}`) +
              " " +
              colors.red(`-${file.linesDeleted}`)
            : "";

          return [
            isSelected ? colors.bold.cyan(marker) : marker,
            statusColor(statusText),
            fileName,
            changes,
          ];
        }),
      )
      .border(false)
      .padding(1);

    lines.push(table.toString());
  }

  private getStatusSymbol(file: FileStatus): string {
    switch (file.status) {
      case "modified":
        return "Modified";
      case "added":
        return "Added";
      case "deleted":
        return "Deleted";
      case "renamed":
        return "Renamed";
      case "untracked":
        return "Untracked";
      default:
        return file.status;
    }
  }

  private renderHelpMenu(lines: string[]) {
    const width = 50;
    const border = "═".repeat(width);
    
    lines.push("");
    lines.push(colors.bold.cyan("╔" + border + "╗"));
    lines.push(
      colors.bold.cyan("║") +
        colors.bold.white(" ".repeat(18) + "HELP MENU") +
        " ".repeat(23) +
        colors.bold.cyan("║"),
    );
    lines.push(colors.bold.cyan("╠" + border + "╣"));
    
    const keybindings = [
      [`${this.config.keybindings.quit}, Esc`, "Quit application"],
      [`${this.config.keybindings.refresh}`, "Refresh git status"],
      [`↑/k`, "Move selection up"],
      [`↓/j`, "Move selection down"],
      [`PgUp`, "Move up 10 items"],
      [`PgDn`, "Move down 10 items"],
      [`${this.config.keybindings.help}`, "Toggle this help menu"],
    ];

    keybindings.forEach(([key, desc]) => {
      const keyPart = colors.bold.yellow(key.padEnd(15));
      const descPart = desc.padEnd(32);
      lines.push(
        colors.bold.cyan("║") + " " + keyPart + descPart + " " +
          colors.bold.cyan("║"),
      );
    });

    lines.push(colors.bold.cyan("╠" + border + "╣"));
    lines.push(
      colors.bold.cyan("║") +
        colors.dim(" Press ? or Esc to close".padEnd(width)) +
        colors.bold.cyan("║"),
    );
    lines.push(colors.bold.cyan("╚" + border + "╝"));
  }

  stop() {
    this.running = false;
    
    // Stop file watcher
    if (this.watcherService) {
      this.watcherService.stop();
    }

    // Restore terminal
    Deno.stdin.setRaw(false);
    Deno.stdout.writeSync(new TextEncoder().encode(SHOW_CURSOR));
    
    // Clear screen
    Deno.stdout.writeSync(
      new TextEncoder().encode(CLEAR_SCREEN + MOVE_CURSOR_HOME),
    );
    
    Deno.exit(0);
  }
}

async function main() {
  const { options, args } = await new Command()
    .name("gfm")
    .version("0.1.0")
    .description(
      "Git Changes Monitor - Real-time TUI for Git repository changes",
    )
    .arguments("[directory:string]")
    .option("-c, --config <path:string>", "Custom config file path")
    .option("--no-watch", "Single snapshot mode (no live updates)")
    .parse(Deno.args);

  // Determine repository path
  const repoPath = args[0] || Deno.cwd();

  // Validate it's a Git repository
  const gitService = new GitService(repoPath);
  const isGitRepo = await gitService.isGitRepo();

  if (!isGitRepo) {
    console.error(`Error: ${repoPath} is not a Git repository`);
    console.error(
      "\nPlease run this command from within a Git repository or provide a path to one.",
    );
    Deno.exit(1);
  }

  // Determine config path
  const configPath = options.config || ConfigService.getDefaultConfigPath();

  // Create and start app
  const app = new GitMonitorApp(repoPath, configPath, options.watch !== false);
  
  // Handle Ctrl+C gracefully
  Deno.addSignalListener("SIGINT", () => {
    app.stop();
  });

  await app.start();
}

// Run the application
if (import.meta.main) {
  main().catch((error) => {
    // Restore terminal on error
    Deno.stdin.setRaw(false);
    Deno.stdout.writeSync(new TextEncoder().encode(SHOW_CURSOR));
    console.error("Fatal error:", error);
    Deno.exit(1);
  });
}
