use hark_collector::collector::Collector;
use hark_core::model::SystemSnapshot;

use crate::ui;
use iced::time;
use iced::{Element, Subscription, Task};
use std::time::Duration;

pub struct Hark {
    snapshot: SystemSnapshot,
    collector: Collector,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

impl Hark {
    pub fn new() -> (Self, Task<Message>) {
        let mut collector = Collector::new();
        let snapshot = collector.snapshot();

        (
            Self {
                snapshot,
                collector,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.snapshot = self.collector.snapshot();
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        ui::render(&self.snapshot)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
