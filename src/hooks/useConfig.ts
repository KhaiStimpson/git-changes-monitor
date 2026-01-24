import { useState, useEffect, useCallback } from "react";
import type { Config } from "../types/config.ts";
import { ConfigService } from "../services/config.ts";
import { expandPath } from "../utils/path-utils.ts";

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

/**
 * Hook for loading and accessing configuration with runtime reloading
 * @param configPath Path to configuration file
 * @returns Configuration object that updates when file changes
 */
export function useConfig(configPath: string): Config {
  const [config, setConfig] = useState<Config>(DEFAULT_CONFIG);

  // Load config function
  const loadConfig = useCallback(async () => {
    const configService = new ConfigService();
    const loaded = await configService.load(configPath);
    setConfig(loaded);
  }, [configPath]);

  // Initial load
  useEffect(() => {
    loadConfig();
  }, [loadConfig]);

  // Watch config file for changes (runtime theme switching)
  useEffect(() => {
    let watcher: Deno.FsWatcher | null = null;

    const startWatching = async () => {
      try {
        const expandedPath = expandPath(configPath);
        watcher = Deno.watchFs(expandedPath);

        for await (const event of watcher) {
          if (event.kind === "modify" || event.kind === "create") {
            // Debounce slightly to avoid rapid reloads
            setTimeout(() => {
              loadConfig();
            }, 100);
          }
        }
      } catch {
        // Config file might not exist yet, that's ok
      }
    };

    startWatching();

    return () => {
      if (watcher) {
        watcher.close();
      }
    };
  }, [configPath, loadConfig]);

  return config;
}
