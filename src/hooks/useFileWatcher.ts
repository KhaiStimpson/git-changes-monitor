import { useEffect } from "react";
import { FileWatcherService } from "../services/watcher.ts";

/**
 * Hook for watching file system changes
 * @param repoPath Path to repository to watch
 * @param onChange Callback when files change
 * @param enabled Whether watching is enabled
 * @param debounceMs Debounce delay in milliseconds
 */
export function useFileWatcher(
  repoPath: string,
  onChange: () => void,
  enabled: boolean = true,
  debounceMs: number = 100,
): void {
  useEffect(() => {
    if (!enabled) return;

    const watcher = new FileWatcherService(repoPath);
    watcher.watch(onChange, debounceMs);

    return () => {
      watcher.stop();
    };
  }, [repoPath, onChange, enabled, debounceMs]);
}
