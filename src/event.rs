use std::time::Duration;

use color_eyre::eyre::{OptionExt, Result};
use crossterm::event::{Event as CrosstermEvent, EventStream};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc::{self, UnboundedSender};

const TICK_PER_SECOND: usize = 24;

pub struct EventHandler {
    sender: mpsc::UnboundedSender<Event>,
    receiver: mpsc::UnboundedReceiver<Event>,
}

pub enum Event {
    Tick,
    App(AppEvent),
    Crossterm(CrosstermEvent),
}

pub enum AppEvent {
    BeszelUpdate
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
        let mut ticker = tokio::time::interval(Duration::from_secs_f32(1.0 / TICK_PER_SECOND as f32));
        let mut beszel_ticker = tokio::time::interval(Duration::from_secs_f32(15.0));
        loop {
            tokio::select! {
                _ = self.sender.closed() => {
                    break
                }
                _ = ticker.tick() => {
                    self.sender.send(Event::Tick)?
                }
                _ = beszel_ticker.tick() => {
                    self.sender.send(Event::App(AppEvent::BeszelUpdate))?
                }
                Some(Ok(e)) = term_reader.next().fuse() => {
                    self.sender.send(Event::Crossterm(e))?
                }

            }
        }
        Ok(())
    }
}
