use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{Event as CrosstermEvent, KeyEvent, KeyEventKind};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

/// Application events
#[derive(Debug, Clone)]
pub enum Event {
    /// Terminal tick (for periodic updates)
    Tick,
    /// Keyboard input
    Key(KeyEvent),
    /// Terminal resize
    #[allow(dead_code)]
    Resize(u16, u16),
    /// File system change detected
    FileChange,
    /// Diff content is ready
    DiffReady(String),
}

/// Event handler that manages event polling and distribution
pub struct EventHandler {
    /// Event sender
    sender: mpsc::UnboundedSender<Event>,
    /// Event receiver
    receiver: mpsc::UnboundedReceiver<Event>,
    /// Task handle for the event loop
    #[allow(dead_code)]
    task: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    /// Create a new event handler with the specified tick rate
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let sender_clone = sender.clone();

        let task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_rate);

            loop {
                let tick = tick_interval.tick();
                let crossterm_event = reader.next().fuse();

                tokio::select! {
                    _ = tick => {
                        if sender_clone.send(Event::Tick).is_err() {
                            break;
                        }
                    }
                    Some(Ok(evt)) = crossterm_event => {
                        match evt {
                            CrosstermEvent::Key(key) => {
                                if key.kind == KeyEventKind::Press {
                                    if sender_clone.send(Event::Key(key)).is_err() {
                                        break;
                                    }
                                }
                            }
                            CrosstermEvent::Resize(w, h) => {
                                if sender_clone.send(Event::Resize(w, h)).is_err() {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        Self {
            sender,
            receiver,
            task,
        }
    }

    /// Get a clone of the event sender for sending events from other parts of the application
    pub fn sender(&self) -> mpsc::UnboundedSender<Event> {
        self.sender.clone()
    }

    /// Wait for the next event
    pub async fn next(&mut self) -> Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or_else(|| color_eyre::eyre::eyre!("Event channel closed"))
    }
}
