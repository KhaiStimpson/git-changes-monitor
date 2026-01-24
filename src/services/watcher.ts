import { debounce } from "../utils/debounce.ts";

export class FileWatcherService {
  private watcher?: Deno.FsWatcher;
  private abortController?: AbortController;

  constructor(private repoPath: string) {}

  /**
   * Starts watching the repository for file changes
   * @param callback Function to call when changes are detected
   * @param debounceMs Debounce delay in milliseconds
   */
  watch(callback: () => void, debounceMs: number = 100): void {
    this.stop();

    this.abortController = new AbortController();
    const debouncedCallback = debounce(callback, debounceMs);

    try {
      this.watcher = Deno.watchFs(this.repoPath, { recursive: true });

      (async () => {
        try {
          for await (const event of this.watcher!) {
            if (this.abortController?.signal.aborted) break;

            // Filter out .git/ directory changes to avoid infinite loops
            const relevantPaths = event.paths.filter(
              (p) => !p.includes("/.git/") && !p.includes("\\.git\\"),
            );

            if (relevantPaths.length > 0) {
              debouncedCallback();
            }
          }
        } catch (error) {
          // Ignore errors from closed watcher
          if (!this.abortController?.signal.aborted) {
            console.error("File watcher error:", error);
          }
        }
      })();
    } catch (error) {
      console.error("Failed to start file watcher:", error);
    }
  }

  /**
   * Stops watching for file changes
   */
  stop(): void {
    this.abortController?.abort();
    try {
      this.watcher?.close();
    } catch {
      // Ignore errors when closing
    }
    this.watcher = undefined;
    this.abortController = undefined;
  }
}
