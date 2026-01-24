import { useState, useEffect, useCallback } from "react";
import type { GitStatus } from "../types/git.ts";
import { GitService } from "../services/git.ts";

/**
 * Hook for fetching and refreshing Git status
 * @param repoPath Path to Git repository
 * @returns Git status and refresh function
 */
export function useGitStatus(repoPath: string) {
  const [status, setStatus] = useState<GitStatus>({
    branch: { name: "Loading...", ahead: 0, behind: 0 },
    stagedFiles: [],
    unstagedFiles: [],
  });
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      const gitService = new GitService(repoPath);
      const newStatus = await gitService.getStatus();
      setStatus(newStatus);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Unknown error");
    } finally {
      setIsLoading(false);
    }
  }, [repoPath]);

  useEffect(() => {
    refresh();
  }, [refresh]);

  return { status, isLoading, error, refresh };
}
