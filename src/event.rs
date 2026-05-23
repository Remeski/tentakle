use color_eyre::eyre::{OptionExt, Result};
use crossterm::event::{Event as CrosstermEvent, EventStream};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc::{self, UnboundedSender};

pub struct EventHandler {
    sender: mpsc::UnboundedSender<Event>,
    receiver: mpsc::UnboundedReceiver<Event>,
}

pub enum Event {
    App,
    Crossterm(CrosstermEvent),
}

impl EventHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let task = EventTask::new(sender.clone());
        tokio::spawn(task.run());
        Self { sender, receiver }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.receiver.recv().await.ok_or_eyre("Bad")
    }
}

pub struct EventTask {
    sender: UnboundedSender<Event>,
}

impl EventTask {
    fn new(sender: UnboundedSender<Event>) -> Self {
        Self { sender }
    }

    async fn run(self) -> Result<()> {
        let mut term_reader = EventStream::new();
        loop {
            tokio::select! {
                _ = self.sender.closed() => {
                    break
                }
                Some(Ok(e)) = term_reader.next().fuse() => {
                    self.sender.send(Event::Crossterm(e))?
                }

            }
        }
        Ok(())
    }
}
