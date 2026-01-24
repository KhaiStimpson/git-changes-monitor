use serde::{Deserialize, Serialize};

/// Type of file status in git
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileStatusType {
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Untracked,
    Unmerged,
}

impl FileStatusType {
    /// Get the short code for this status type
    pub fn code(&self) -> &'static str {
        match self {
            FileStatusType::Modified => "M",
            FileStatusType::Added => "A",
            FileStatusType::Deleted => "D",
            FileStatusType::Renamed => "R",
            FileStatusType::Copied => "C",
            FileStatusType::Untracked => "?",
            FileStatusType::Unmerged => "U",
        }
    }

    /// Parse a status code from git porcelain v2 output
    pub fn from_code(code: char) -> Option<Self> {
        match code {
            'M' => Some(FileStatusType::Modified),
            'A' => Some(FileStatusType::Added),
            'D' => Some(FileStatusType::Deleted),
            'R' => Some(FileStatusType::Renamed),
            'C' => Some(FileStatusType::Copied),
            '?' => Some(FileStatusType::Untracked),
            'U' => Some(FileStatusType::Unmerged),
            _ => None,
        }
    }
}

/// Status of a single file in the repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStatus {
    /// Path to the file (relative to repo root)
    pub path: String,
    /// Type of change
    pub status: FileStatusType,
    /// Whether this file is staged
    pub staged: bool,
    /// Number of lines added
    pub lines_added: usize,
    /// Number of lines deleted
    pub lines_deleted: usize,
    /// Original path (for renames)
    pub old_path: Option<String>,
}

impl FileStatus {
    /// Create a new file status
    pub fn new(path: String, status: FileStatusType, staged: bool) -> Self {
        Self {
            path,
            status,
            staged,
            lines_added: 0,
            lines_deleted: 0,
            old_path: None,
        }
    }
}

/// Information about the current branch
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BranchInfo {
    /// Name of the current branch
    pub name: String,
    /// Name of the upstream branch (if any)
    pub upstream: Option<String>,
    /// Number of commits ahead of upstream
    pub ahead: usize,
    /// Number of commits behind upstream
    pub behind: usize,
}

/// Information about the last commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    /// Short hash of the commit
    pub hash: String,
    /// Author name
    pub author: String,
    /// Commit subject (first line of message)
    pub subject: String,
}

/// Complete git status for a repository
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GitStatus {
    /// Branch information
    pub branch: BranchInfo,
    /// Last commit information
    pub last_commit: Option<CommitInfo>,
    /// List of staged files
    pub staged_files: Vec<FileStatus>,
    /// List of unstaged files
    pub unstaged_files: Vec<FileStatus>,
}

impl GitStatus {
    /// Get the total number of changed files
    #[allow(dead_code)]
    pub fn total_files(&self) -> usize {
        self.staged_files.len() + self.unstaged_files.len()
    }

    /// Check if there are any changes
    #[allow(dead_code)]
    pub fn has_changes(&self) -> bool {
        !self.staged_files.is_empty() || !self.unstaged_files.is_empty()
    }
}
