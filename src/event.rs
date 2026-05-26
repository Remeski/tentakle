use std::time::Duration;

use color_eyre::eyre::{OptionExt, Result};
use crossterm::event::{Event as CrosstermEvent, EventStream};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc::{self, UnboundedSender};

use crate::{app::App, integrations::ntfy::NtfyMessage};

const TICK_PER_SECOND: usize = 24;

pub struct EventHandler {
    pub sender: mpsc::UnboundedSender<Event>,
    receiver: mpsc::UnboundedReceiver<Event>,
}

pub enum Event {
    Tick,
    App(AppEvent),
    Crossterm(CrosstermEvent),
}

pub enum AppEvent {
    Beszel(BeszelEvent),
    Ntfy(NtfyEvent),
    ClearMsg
}

pub enum BeszelEvent {
    Initialize,
    Update,
    ChangeLoadAverageHost
}

pub enum NtfyEvent {
    Initialize,
    Update,
    Msg(NtfyMessage),
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

    pub fn send(&self, event: AppEvent) -> Result<()> {
        self.sender.send(Event::App(event))?;
        return Ok(());
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
        loop {
            tokio::select! {
                _ = self.sender.closed() => {
                    break
                }
                _ = ticker.tick() => {
                    self.sender.send(Event::Tick)?
                }
                Some(Ok(e)) = term_reader.next().fuse() => {
                    self.sender.send(Event::Crossterm(e))?
                }

            }
        }
        Ok(())
    }
}

pub trait HandleEvent {
    type Event;
    async fn handle_event(app: &mut App, event: Self::Event) -> Result<()>;
}
