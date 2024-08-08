use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use async_trait::async_trait;
use tokio::sync::mpsc;
use tracing::error;

use super::{SearchFileEvent, Sink};

#[derive(Clone)]
pub struct Console {
    quiet: bool,
    sender: mpsc::UnboundedSender<SearchFileEvent>,
    valid: Arc<AtomicBool>,
}

impl Console {
    #[must_use]
    pub fn new(quiet: bool) -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel::<SearchFileEvent>();
        let valid = Arc::new(AtomicBool::new(true));
        let valid_task = valid.clone();

        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                if valid_task.load(Ordering::Relaxed) {
                    valid_task.swap(false, Ordering::Relaxed);
                }

                if !quiet {
                    let mut path = event.path.to_string_lossy().into_owned();
                    if let Some(line_number) = event.line_number {
                        path.push(':');
                        path.push_str(line_number.to_string().as_str());
                    }
                    eprintln!(
                        "\"{}\" in \"\u{1b}]8;i;file://{path}\u{1b}\\{path}\u{1b}]8;;\u{1b}\\\"",
                        event.class
                    );
                }
            }
        });

        Self {
            quiet,
            sender,
            valid,
        }
    }

    fn is_valid(&self) -> bool {
        self.valid.load(Ordering::Relaxed)
    }
}

#[async_trait]
impl Sink for Console {
    type Event = SearchFileEvent;

    fn send(&mut self, event: Self::Event) {
        if self.sender.send(event).is_err() {
            error!("channel closed");
        }
    }

    async fn done(self) -> bool {
        let valid = self.is_valid();

        if !self.quiet {
            if valid {
                println!("Classes are all valid");
            } else {
                eprintln!("Unknown classes found");
            }
        }

        valid
    }
}
