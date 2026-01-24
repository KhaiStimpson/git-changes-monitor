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

        Ok(Self { debouncer })
    }
}
