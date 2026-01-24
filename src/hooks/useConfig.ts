import { useState, useEffect } from "react";
import type { Config } from "../types/config.ts";
import { ConfigService } from "../services/config.ts";

/**
 * Hook for loading and accessing configuration
 * @param configPath Path to configuration file
 * @returns Configuration object
 */
export function useConfig(configPath: string): Config {
  const [config, setConfig] = useState<Config>({
    display: {
      showFilePathAndStatus: true,
      showLineChangeCounts: true,
      showStagedVsUnstaged: true,
      showFilePreview: true,
      showBranchInfo: true,
      showLastCommitInfo: true,
    },
    ui: {
      colorScheme: "default",
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
  });

  useEffect(() => {
    const configService = new ConfigService();
    configService.load(configPath).then(setConfig);
  }, [configPath]);

  return config;
}
