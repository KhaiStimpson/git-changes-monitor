/**
 * Hook for accessing the current theme
 */

import { useMemo } from "react";
import { getTheme, type Theme, type ThemeName } from "./themes.ts";

/**
 * Hook to get the current theme based on config colorScheme
 * @param colorScheme The color scheme name from config
 * @returns The theme object with all colors
 */
export function useTheme(colorScheme: string): Theme {
  return useMemo(() => getTheme(colorScheme), [colorScheme]);
}

// Re-export types for convenience
export type { Theme, ThemeName };
