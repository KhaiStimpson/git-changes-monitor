use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;

use color_eyre::eyre::{eyre, Result};
use tokio::process::Command;

use super::types::{BranchInfo, CommitInfo, FileStatus, FileStatusType, GitStatus};

/// Service for executing git commands and parsing their output
pub struct GitService {
    /// Path to the repository
    repo_path: PathBuf,
}

impl GitService {
    /// Create a new git service for the given repository path
    pub fn new(repo_path: PathBuf) -> Self {
        Self { repo_path }
    }

    /// Check if the path is a valid git repository
    pub async fn is_git_repo(&self) -> Result<bool> {
        let output = Command::new("git")
            .args(["rev-parse", "--git-dir"])
            .current_dir(&self.repo_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await?;

        Ok(output.success())
    }

    /// Get the complete git status for the repository
    pub async fn get_status(&self) -> Result<GitStatus> {
        // Run multiple git commands in parallel
        let (branch_result, commit_result, status_result, diff_staged_result, diff_unstaged_result) = tokio::join!(
            self.get_branch_info(),
            self.get_last_commit(),
            self.get_file_statuses(),
            self.get_diff_stats(true),
            self.get_diff_stats(false),
        );

        let branch = branch_result.unwrap_or_default();
        let last_commit = commit_result.ok();
        let (mut staged_files, mut unstaged_files) = status_result?;

        // Merge diff stats
        let staged_stats = diff_staged_result.unwrap_or_default();
        let unstaged_stats = diff_unstaged_result.unwrap_or_default();

        for file in &mut staged_files {
            if let Some((added, deleted)) = staged_stats.get(&file.path) {
                file.lines_added = *added;
                file.lines_deleted = *deleted;
            }
        }

        for file in &mut unstaged_files {
            if let Some((added, deleted)) = unstaged_stats.get(&file.path) {
                file.lines_added = *added;
                file.lines_deleted = *deleted;
            }
        }

        Ok(GitStatus {
            branch,
            last_commit,
            staged_files,
            unstaged_files,
        })
    }

    /// Get branch information
    async fn get_branch_info(&self) -> Result<BranchInfo> {
        let output = Command::new("git")
            .args(["status", "-sb", "--porcelain=v2"])
            .current_dir(&self.repo_path)
            .output()
            .await?;

        if !output.status.success() {
            return Err(eyre!("Failed to get branch info"));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut info = BranchInfo::default();

        for line in stdout.lines() {
            if line.starts_with("# branch.head ") {
                info.name = line.trim_start_matches("# branch.head ").to_string();
            } else if line.starts_with("# branch.upstream ") {
                info.upstream = Some(line.trim_start_matches("# branch.upstream ").to_string());
            } else if line.starts_with("# branch.ab ") {
                let ab = line.trim_start_matches("# branch.ab ");
                let parts: Vec<&str> = ab.split_whitespace().collect();
                if parts.len() >= 2 {
                    info.ahead = parts[0].trim_start_matches('+').parse().unwrap_or(0);
                    info.behind = parts[1].trim_start_matches('-').parse().unwrap_or(0);
                }
            }
        }

        Ok(info)
    }

    /// Get last commit information
    async fn get_last_commit(&self) -> Result<CommitInfo> {
        let output = Command::new("git")
            .args(["log", "-1", "--pretty=format:%h|%an|%s"])
            .current_dir(&self.repo_path)
            .output()
            .await?;

        if !output.status.success() {
            return Err(eyre!("Failed to get last commit"));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = stdout.splitn(3, '|').collect();

        if parts.len() < 3 {
            return Err(eyre!("Invalid commit format"));
        }

        Ok(CommitInfo {
            hash: parts[0].to_string(),
            author: parts[1].to_string(),
            subject: parts[2].to_string(),
        })
    }

    /// Get file statuses (staged and unstaged)
    async fn get_file_statuses(&self) -> Result<(Vec<FileStatus>, Vec<FileStatus>)> {
        let output = Command::new("git")
            .args(["status", "--porcelain=v2"])
            .current_dir(&self.repo_path)
            .output()
            .await?;

        if !output.status.success() {
            return Err(eyre!("Failed to get file statuses"));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut staged = Vec::new();
        let mut unstaged = Vec::new();

        for line in stdout.lines() {
            if line.starts_with("1 ") || line.starts_with("2 ") {
                // Changed entries
                self.parse_changed_entry(line, &mut staged, &mut unstaged);
            } else if line.starts_with("? ") {
                // Untracked files
                let path = line[2..].to_string();
                unstaged.push(FileStatus::new(path, FileStatusType::Untracked, false));
            } else if line.starts_with("u ") {
                // Unmerged entries
                self.parse_unmerged_entry(line, &mut unstaged);
            }
        }

        Ok((staged, unstaged))
    }

    /// Parse a changed entry from porcelain v2 output
    fn parse_changed_entry(
        &self,
        line: &str,
        staged: &mut Vec<FileStatus>,
        unstaged: &mut Vec<FileStatus>,
    ) {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() < 9 {
            return;
        }

        let xy = parts[1];
        let path = if line.starts_with("2 ") {
            // Rename entry has path at the end after a tab
            if let Some(tab_pos) = line.find('\t') {
                let paths = &line[tab_pos + 1..];
                let path_parts: Vec<&str> = paths.split('\t').collect();
                if path_parts.len() >= 2 {
                    path_parts[1].to_string()
                } else {
                    parts[9..].join(" ")
                }
            } else {
                parts[9..].join(" ")
            }
        } else {
            parts[8..].join(" ")
        };

        let index_status = xy.chars().next().unwrap_or('.');
        let worktree_status = xy.chars().nth(1).unwrap_or('.');

        // Staged changes (index)
        if index_status != '.' {
            if let Some(status_type) = FileStatusType::from_code(index_status) {
                let mut file = FileStatus::new(path.clone(), status_type, true);
                if line.starts_with("2 ") {
                    // Handle rename - extract old path
                    if let Some(tab_pos) = line.find('\t') {
                        let paths = &line[tab_pos + 1..];
                        let path_parts: Vec<&str> = paths.split('\t').collect();
                        if !path_parts.is_empty() {
                            file.old_path = Some(path_parts[0].to_string());
                        }
                    }
                }
                staged.push(file);
            }
        }

        // Unstaged changes (worktree)
        if worktree_status != '.' {
            if let Some(status_type) = FileStatusType::from_code(worktree_status) {
                unstaged.push(FileStatus::new(path, status_type, false));
            }
        }
    }

    /// Parse an unmerged entry from porcelain v2 output
    fn parse_unmerged_entry(&self, line: &str, unstaged: &mut Vec<FileStatus>) {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() >= 11 {
            let path = parts[10..].join(" ");
            unstaged.push(FileStatus::new(path, FileStatusType::Unmerged, false));
        }
    }

    /// Get diff stats (lines added/deleted per file)
    async fn get_diff_stats(&self, staged: bool) -> Result<HashMap<String, (usize, usize)>> {
        let mut args = vec!["diff", "--numstat"];
        if staged {
            args.push("--cached");
        }

        let output = Command::new("git")
            .args(&args)
            .current_dir(&self.repo_path)
            .output()
            .await?;

        if !output.status.success() {
            return Ok(HashMap::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut stats = HashMap::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 {
                let added: usize = parts[0].parse().unwrap_or(0);
                let deleted: usize = parts[1].parse().unwrap_or(0);
                let path = parts[2].to_string();
                stats.insert(path, (added, deleted));
            }
        }

        Ok(stats)
    }

    /// Get the diff content for a specific file
    pub async fn get_file_diff(&self, path: &str, staged: bool) -> Result<String> {
        let mut args = vec!["diff"];
        if staged {
            args.push("--cached");
        }
        args.push("--");
        args.push(path);

        let output = Command::new("git")
            .args(&args)
            .current_dir(&self.repo_path)
            .output()
            .await?;

        if !output.status.success() {
            return Err(eyre!("Failed to get diff for {}", path));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
