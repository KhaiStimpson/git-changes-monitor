export type FileStatusType =
  | "modified"
  | "added"
  | "deleted"
  | "renamed"
  | "copied"
  | "untracked"
  | "unmerged";

export interface FileStatus {
  path: string;
  status: FileStatusType;
  staged: boolean;
  linesAdded: number;
  linesDeleted: number;
  oldPath?: string; // For renames
}

export interface BranchInfo {
  name: string;
  upstream?: string;
  ahead: number;
  behind: number;
}

export interface CommitInfo {
  hash: string;
  author: string;
  subject: string;
}

export interface DiffStat {
  added: number;
  deleted: number;
}

export interface GitStatus {
  branch: BranchInfo;
  lastCommit?: CommitInfo;
  stagedFiles: FileStatus[];
  unstagedFiles: FileStatus[];
}
