export interface DisplayConfig {
  showFilePathAndStatus: boolean;
  showLineChangeCounts: boolean;
  showStagedVsUnstaged: boolean;
  showFilePreview: boolean;
  showBranchInfo: boolean;
  showLastCommitInfo: boolean;
}

export type ColorScheme = "catppuccin" | "nord" | "tokyo-night" | "dracula";

export interface UIConfig {
  colorScheme: ColorScheme;
  refreshDebounceMs: number;
  maxPreviewLines: number;
}

export interface KeybindingsConfig {
  quit: string;
  refresh: string;
  togglePreview: string;
  help: string;
  up: string;
  down: string;
  pageUp: string;
  pageDown: string;
}

export interface Config {
  display: DisplayConfig;
  ui: UIConfig;
  keybindings: KeybindingsConfig;
}
