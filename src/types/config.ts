export interface DisplayConfig {
  showFilePathAndStatus: boolean;
  showLineChangeCounts: boolean;
  showStagedVsUnstaged: boolean;
  showFilePreview: boolean;
  showBranchInfo: boolean;
  showLastCommitInfo: boolean;
}

export interface UIConfig {
  colorScheme: "default" | "dark" | "light";
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
