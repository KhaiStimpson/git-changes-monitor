use std::path::PathBuf;
use std::time::Duration;

use color_eyre::eyre::Result;
use notify::{Config, RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind, Debouncer};
use tokio::sync::mpsc;

use crate::event::Event;

/// File watcher service that monitors the repository for changes
pub struct FileWatcher {
    /// The debouncer wrapping the file watcher
    #[allow(dead_code)]
    debouncer: Debouncer<RecommendedWatcher>,
    /// Separate debouncer for git internal files (HEAD, refs)
    #[allow(dead_code)]
    git_debouncer: Debouncer<RecommendedWatcher>,
}

impl FileWatcher {
    /// Create a new file watcher for the given repository path
    pub fn new(
        repo_path: PathBuf,
        event_tx: mpsc::UnboundedSender<Event>,
        debounce_duration: Duration,
    ) -> Result<Self> {
        let tx = event_tx.clone();

        let mut debouncer = new_debouncer(
            debounce_duration,
            move |result: Result<Vec<notify_debouncer_mini::DebouncedEvent>, notify::Error>| {
                match result {
                    Ok(events) => {
                        // Check if any event is relevant (not just metadata changes)
                        let has_relevant_event = events
                            .iter()
                            .any(|e| matches!(e.kind, DebouncedEventKind::Any));

                        if has_relevant_event {
                            // Send file change event
                            let _ = tx.send(Event::FileChange);
                        }
                    }
                    Err(_e) => {
                        // Ignore watch errors silently
                    }
                }
            },
        )?;

        // Watch the repository directory recursively
        debouncer
            .watcher()
            .configure(Config::default().with_poll_interval(Duration::from_millis(100)))?;

        debouncer
            .watcher()
            .watch(&repo_path, RecursiveMode::Recursive)?;

        // Create a separate watcher for git internal files (branch/commit changes)
        let git_tx = event_tx.clone();
        let mut git_debouncer = new_debouncer(
            debounce_duration,
            move |result: Result<Vec<notify_debouncer_mini::DebouncedEvent>, notify::Error>| {
                match result {
                    Ok(events) => {
                        let has_relevant_event = events
                            .iter()
                            .any(|e| matches!(e.kind, DebouncedEventKind::Any));

                        if has_relevant_event {
                            let _ = git_tx.send(Event::FileChange);
                        }
                    }
                    Err(_e) => {
                        // Ignore watch errors silently
                    }
                }
            },
        )?;

        git_debouncer
            .watcher()
            .configure(Config::default().with_poll_interval(Duration::from_millis(100)))?;

        // Watch .git/HEAD for branch switches
        let git_head = repo_path.join(".git").join("HEAD");
        if git_head.exists() {
            let _ = git_debouncer
                .watcher()
                .watch(&git_head, RecursiveMode::NonRecursive);
        }

        // Watch .git/refs/heads for new commits on local branches
        let git_refs_heads = repo_path.join(".git").join("refs").join("heads");
        if git_refs_heads.exists() {
            let _ = git_debouncer
                .watcher()
                .watch(&git_refs_heads, RecursiveMode::Recursive);
        }

        // Watch .git/refs/remotes for remote tracking branch updates (after fetch/pull)
        let git_refs_remotes = repo_path.join(".git").join("refs").join("remotes");
        if git_refs_remotes.exists() {
            let _ = git_debouncer
                .watcher()
                .watch(&git_refs_remotes, RecursiveMode::Recursive);
        }

        // Watch .git/index for staging area changes
        let git_index = repo_path.join(".git").join("index");
        if git_index.exists() {
            let _ = git_debouncer
                .watcher()
                .watch(&git_index, RecursiveMode::NonRecursive);
        }

        // Watch .git/packed-refs for packed reference updates
        let git_packed_refs = repo_path.join(".git").join("packed-refs");
        if git_packed_refs.exists() {
            let _ = git_debouncer
                .watcher()
                .watch(&git_packed_refs, RecursiveMode::NonRecursive);
        }

        Ok(Self {
            debouncer,
            git_debouncer,
        })
    }
}
