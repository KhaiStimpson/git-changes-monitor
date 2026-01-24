/**
 * Theme definitions for the Git Changes Monitor TUI
 * Supports: Catppuccin Mocha, Nord, Tokyo Night, Dracula
 */

export type ThemeName = "catppuccin" | "nord" | "tokyo-night" | "dracula";

export interface Theme {
  name: ThemeName;
  displayName: string;

  // Base colors
  base: string; // Main background
  surface: string; // Section backgrounds (slightly lighter)
  overlay: string; // Modal/popup backgrounds

  // Text colors
  text: string; // Primary text
  subtext: string; // Dimmed/secondary text

  // Accent colors
  accent: string; // Primary accent (headers, highlights)
  accentSecondary: string; // Secondary accent

  // Semantic colors
  success: string; // Added, success states
  error: string; // Deleted, error states
  warning: string; // Modified, warnings
  info: string; // Info, renamed, copied

  // Git-specific
  staged: string; // Staged indicator
  unstaged: string; // Unstaged indicator
  untracked: string; // Untracked files

  // Selection
  selection: string; // Selected item background
  selectionText: string; // Selected item text

  // Borders (for help menu)
  border: string;
}

/**
 * Catppuccin Mocha Theme
 * Warm, pastel colors with purple/pink accents
 * https://github.com/catppuccin/catppuccin
 */
export const catppuccin: Theme = {
  name: "catppuccin",
  displayName: "Catppuccin Mocha",

  // Base colors
  base: "#1e1e2e",
  surface: "#313244",
  overlay: "#45475a",

  // Text colors
  text: "#cdd6f4",
  subtext: "#a6adc8",

  // Accent colors
  accent: "#cba6f7", // Mauve
  accentSecondary: "#f5c2e7", // Pink

  // Semantic colors
  success: "#a6e3a1", // Green
  error: "#f38ba8", // Red
  warning: "#f9e2af", // Yellow
  info: "#89b4fa", // Blue

  // Git-specific
  staged: "#94e2d5", // Teal
  unstaged: "#fab387", // Peach
  untracked: "#6c7086", // Overlay0

  // Selection
  selection: "#45475a",
  selectionText: "#cdd6f4",

  // Borders
  border: "#585b70",
};

/**
 * Nord Theme
 * Cool blue-grey tones with cyan accents
 * https://www.nordtheme.com/
 */
export const nord: Theme = {
  name: "nord",
  displayName: "Nord",

  // Base colors
  base: "#2e3440",
  surface: "#3b4252",
  overlay: "#434c5e",

  // Text colors
  text: "#eceff4",
  subtext: "#d8dee9",

  // Accent colors
  accent: "#88c0d0", // Frost
  accentSecondary: "#81a1c1", // Frost darker

  // Semantic colors
  success: "#a3be8c", // Aurora green
  error: "#bf616a", // Aurora red
  warning: "#ebcb8b", // Aurora yellow
  info: "#5e81ac", // Frost deep

  // Git-specific
  staged: "#8fbcbb", // Frost teal
  unstaged: "#d08770", // Aurora orange
  untracked: "#4c566a", // Polar night

  // Selection
  selection: "#434c5e",
  selectionText: "#eceff4",

  // Borders
  border: "#4c566a",
};

/**
 * Tokyo Night Theme
 * Dark blue background with vibrant purple/magenta accents
 * https://github.com/enkia/tokyo-night-vscode-theme
 */
export const tokyoNight: Theme = {
  name: "tokyo-night",
  displayName: "Tokyo Night",

  // Base colors
  base: "#1a1b26",
  surface: "#24283b",
  overlay: "#414868",

  // Text colors
  text: "#c0caf5",
  subtext: "#9aa5ce",

  // Accent colors
  accent: "#bb9af7", // Purple
  accentSecondary: "#7dcfff", // Cyan

  // Semantic colors
  success: "#9ece6a", // Green
  error: "#f7768e", // Red
  warning: "#e0af68", // Orange/Yellow
  info: "#7aa2f7", // Blue

  // Git-specific
  staged: "#73daca", // Teal
  unstaged: "#ff9e64", // Orange
  untracked: "#565f89", // Comment

  // Selection
  selection: "#364a82",
  selectionText: "#c0caf5",

  // Borders
  border: "#565f89",
};

/**
 * Dracula Theme
 * Dark purple/grey with pink, cyan, and green accents
 * https://draculatheme.com/
 */
export const dracula: Theme = {
  name: "dracula",
  displayName: "Dracula",

  // Base colors
  base: "#282a36",
  surface: "#44475a",
  overlay: "#6272a4",

  // Text colors
  text: "#f8f8f2",
  subtext: "#bfbfbf",

  // Accent colors
  accent: "#ff79c6", // Pink
  accentSecondary: "#bd93f9", // Purple

  // Semantic colors
  success: "#50fa7b", // Green
  error: "#ff5555", // Red
  warning: "#f1fa8c", // Yellow
  info: "#8be9fd", // Cyan

  // Git-specific
  staged: "#50fa7b", // Green
  unstaged: "#ffb86c", // Orange
  untracked: "#6272a4", // Comment

  // Selection
  selection: "#44475a",
  selectionText: "#f8f8f2",

  // Borders
  border: "#6272a4",
};

/**
 * All available themes
 */
export const themes: Record<ThemeName, Theme> = {
  catppuccin,
  nord,
  "tokyo-night": tokyoNight,
  dracula,
};

/**
 * Get theme by name, with fallback to catppuccin
 */
export function getTheme(name: string): Theme {
  if (name in themes) {
    return themes[name as ThemeName];
  }

  // Backward compatibility: map old values to catppuccin
  if (name === "default" || name === "dark" || name === "light") {
    return catppuccin;
  }

  return catppuccin;
}

/**
 * List of all theme names for validation
 */
export const themeNames: ThemeName[] = [
  "catppuccin",
  "nord",
  "tokyo-night",
  "dracula",
];
