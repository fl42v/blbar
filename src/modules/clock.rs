use chrono::{DateTime, Local};
use iced::{
    widget::{column, container, text},
    Element,
};
use std::time::Duration;

pub struct Clock {
    date: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Update,
}

impl Clock {
    pub fn new() -> Self {
        Self { date: Local::now() }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Update => {
                self.date = Local::now();
            }
        }
    }

    pub fn view(&self, _format: &str) -> Element<Message> {
        column![
            text(self.date.format("%H").to_string()),
            text(self.date.format("%I").to_string()),
        ]
        .padding([0, 0, 0, 2])
        .into()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::time::every(Duration::from_secs(20)).map(|_| Message::Update)
    }
}
