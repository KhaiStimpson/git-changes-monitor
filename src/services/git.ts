import type {
  BranchInfo,
  CommitInfo,
  DiffStat,
  FileStatus,
  FileStatusType,
  GitStatus,
} from "../types/git.ts";

export class GitService {
  constructor(private repoPath: string) {}

  /**
   * Validates if the path is a Git repository
   */
  async isGitRepo(): Promise<boolean> {
    try {
      const cmd = new Deno.Command("git", {
        args: ["rev-parse", "--git-dir"],
        cwd: this.repoPath,
        stdout: "null",
        stderr: "null",
      });
      const { success } = await cmd.output();
      return success;
    } catch {
      return false;
    }
  }

  /**
   * Gets complete Git status including files and branch info
   */
  async getStatus(): Promise<GitStatus> {
    const [branch, lastCommit, files, diffStats, stagedDiffStats] =
      await Promise.all([
        this.getBranchInfo(),
        this.getLastCommit(),
        this.getFileStatuses(),
        this.getDiffStats(false),
        this.getDiffStats(true),
      ]);

    // Merge diff stats into file statuses
    const stagedFiles: FileStatus[] = [];
    const unstagedFiles: FileStatus[] = [];

    for (const file of files) {
      const stats = file.staged
        ? stagedDiffStats.get(file.path)
        : diffStats.get(file.path);

      const fileWithStats: FileStatus = {
        ...file,
        linesAdded: stats?.added ?? 0,
        linesDeleted: stats?.deleted ?? 0,
      };

      if (file.staged) {
        stagedFiles.push(fileWithStats);
      } else {
        unstagedFiles.push(fileWithStats);
      }
    }

    return {
      branch,
      lastCommit,
      stagedFiles,
      unstagedFiles,
    };
  }

  /**
   * Gets file statuses using git status --porcelain=v2
   */
  private async getFileStatuses(): Promise<FileStatus[]> {
    try {
      const cmd = new Deno.Command("git", {
        args: ["status", "--porcelain=v2"],
        cwd: this.repoPath,
        stdout: "piped",
        stderr: "piped",
      });

      const { stdout, success } = await cmd.output();
      if (!success) return [];

      const output = new TextDecoder().decode(stdout);
      const files: FileStatus[] = [];

      for (const line of output.split("\n")) {
        if (!line.trim()) continue;

        // Parse porcelain v2 format
        if (line.startsWith("1 ") || line.startsWith("2 ")) {
          const parts = line.split(" ");
          const xy = parts[1]; // XY status codes
          const path = parts.slice(8).join(" ");

          // X is staged, Y is unstaged
          const stagedCode = xy[0];
          const unstagedCode = xy[1];

          if (stagedCode !== "." && stagedCode !== " ") {
            files.push({
              path,
              status: this.parseStatusCode(stagedCode),
              staged: true,
              linesAdded: 0,
              linesDeleted: 0,
            });
          }

          if (unstagedCode !== "." && unstagedCode !== " ") {
            files.push({
              path,
              status: this.parseStatusCode(unstagedCode),
              staged: false,
              linesAdded: 0,
              linesDeleted: 0,
            });
          }
        } else if (line.startsWith("? ")) {
          // Untracked file
          const path = line.slice(2);
          files.push({
            path,
            status: "untracked",
            staged: false,
            linesAdded: 0,
            linesDeleted: 0,
          });
        }
      }

      return files;
    } catch {
      return [];
    }
  }

  /**
   * Parses Git status code to FileStatusType
   */
  private parseStatusCode(code: string): FileStatusType {
    switch (code) {
      case "M":
        return "modified";
      case "A":
        return "added";
      case "D":
        return "deleted";
      case "R":
        return "renamed";
      case "C":
        return "copied";
      case "U":
        return "unmerged";
      default:
        return "modified";
    }
  }

  /**
   * Gets diff statistics (line changes) for files
   */
  private async getDiffStats(staged: boolean): Promise<Map<string, DiffStat>> {
    try {
      const args = ["diff", "--numstat"];
      if (staged) args.push("--cached");

      const cmd = new Deno.Command("git", {
        args,
        cwd: this.repoPath,
        stdout: "piped",
        stderr: "piped",
      });

      const { stdout, success } = await cmd.output();
      if (!success) return new Map();

      const output = new TextDecoder().decode(stdout);
      const stats = new Map<string, DiffStat>();

      for (const line of output.split("\n")) {
        if (!line.trim()) continue;

        const parts = line.split("\t");
        if (parts.length < 3) continue;

        const added = parseInt(parts[0]) || 0;
        const deleted = parseInt(parts[1]) || 0;
        const path = parts[2];

        stats.set(path, { added, deleted });
      }

      return stats;
    } catch {
      return new Map();
    }
  }

  /**
   * Gets current branch information
   */
  async getBranchInfo(): Promise<BranchInfo> {
    try {
      const cmd = new Deno.Command("git", {
        args: ["status", "-sb", "--porcelain=v2"],
        cwd: this.repoPath,
        stdout: "piped",
        stderr: "piped",
      });

      const { stdout, success } = await cmd.output();
      if (!success) {
        return { name: "unknown", ahead: 0, behind: 0 };
      }

      const output = new TextDecoder().decode(stdout);
      const branchLine = output.split("\n")[0];

      if (!branchLine.startsWith("# branch.head")) {
        return { name: "unknown", ahead: 0, behind: 0 };
      }

      const name = branchLine.split(" ")[2];
      let ahead = 0;
      let behind = 0;
      let upstream: string | undefined;

      // Parse upstream and ahead/behind
      for (const line of output.split("\n")) {
        if (line.startsWith("# branch.upstream")) {
          upstream = line.split(" ")[2];
        } else if (line.startsWith("# branch.ab")) {
          const parts = line.split(" ");
          ahead = parseInt(parts[2].replace("+", "")) || 0;
          behind = parseInt(parts[3].replace("-", "")) || 0;
        }
      }

      return { name, upstream, ahead, behind };
    } catch {
      return { name: "unknown", ahead: 0, behind: 0 };
    }
  }

  /**
   * Gets last commit information
   */
  async getLastCommit(): Promise<CommitInfo | undefined> {
    try {
      const cmd = new Deno.Command("git", {
        args: ["log", "-1", "--pretty=format:%H|%an|%s"],
        cwd: this.repoPath,
        stdout: "piped",
        stderr: "piped",
      });

      const { stdout, success } = await cmd.output();
      if (!success) return undefined;

      const output = new TextDecoder().decode(stdout).trim();
      if (!output) return undefined;

      const [hash, author, subject] = output.split("|");
      return { hash: hash.slice(0, 7), author, subject };
    } catch {
      return undefined;
    }
  }

  /**
   * Gets diff for a specific file
   */
  async getFileDiff(path: string, staged: boolean = false): Promise<string> {
    try {
      const args = ["diff"];
      if (staged) args.push("--cached");
      args.push("--", path);

      const cmd = new Deno.Command("git", {
        args,
        cwd: this.repoPath,
        stdout: "piped",
        stderr: "piped",
      });

      const { stdout, success } = await cmd.output();
      if (!success) return "";

      return new TextDecoder().decode(stdout);
    } catch {
      return "";
    }
  }
}
