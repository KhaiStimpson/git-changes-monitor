import type { Config } from "../types/config.ts";
import { dirname, expandPath } from "../utils/path-utils.ts";

const DEFAULT_CONFIG: Config = {
  display: {
    showFilePathAndStatus: true,
    showLineChangeCounts: true,
    showStagedVsUnstaged: true,
    showFilePreview: true,
    showBranchInfo: true,
    showLastCommitInfo: true,
  },
  ui: {
    colorScheme: "catppuccin",
    refreshDebounceMs: 100,
    maxPreviewLines: 20,
  },
  keybindings: {
    quit: "q",
    refresh: "r",
    togglePreview: "p",
    help: "?",
    up: "up",
    down: "down",
    pageUp: "pageup",
    pageDown: "pagedown",
  },
};

export class ConfigService {
  private config: Config = DEFAULT_CONFIG;

  /**
   * Loads configuration from file, creating default if it doesn't exist
   * @param configPath Path to config file
   * @returns Loaded configuration
   */
  async load(configPath: string): Promise<Config> {
    try {
      const expandedPath = expandPath(configPath);
      const content = await Deno.readTextFile(expandedPath);
      const userConfig = JSON.parse(content);

      // Deep merge with defaults
      this.config = this.mergeConfig(DEFAULT_CONFIG, userConfig);
      return this.config;
    } catch (error) {
      // If file doesn't exist, create it with defaults
      if (error instanceof Deno.errors.NotFound) {
        await this.createDefault(configPath);
      }
      return DEFAULT_CONFIG;
    }
  }

  /**
   * Gets the current configuration
   */
  get(): Config {
    return this.config;
  }

  /**
   * Creates default configuration file
   * @param configPath Path to create config file at
   */
  private async createDefault(configPath: string): Promise<void> {
    try {
      const expandedPath = expandPath(configPath);
      const dir = dirname(expandedPath);

      // Create directory if it doesn't exist
      await Deno.mkdir(dir, { recursive: true });

      // Write default config
      await Deno.writeTextFile(
        expandedPath,
        JSON.stringify(DEFAULT_CONFIG, null, 2),
      );
    } catch (error) {
      console.warn("Failed to create default config:", error);
    }
  }

  /**
   * Deep merges user config with defaults
   */
  private mergeConfig(defaults: Config, user: Partial<Config>): Config {
    return {
      display: { ...defaults.display, ...user.display },
      ui: { ...defaults.ui, ...user.ui },
      keybindings: { ...defaults.keybindings, ...user.keybindings },
    };
  }

  /**
   * Gets default config path
   */
  static getDefaultConfigPath(): string {
    const home = Deno.env.get("HOME") || Deno.env.get("USERPROFILE");
    if (!home) {
      throw new Error("Could not determine home directory");
    }
    return `${home}/.config/git-file-monitor/gfm.json`;
  }
}
