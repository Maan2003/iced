use std::{cell::RefCell, time::Instant};

use iced::{
    button, Alignment, Button, Column, Element, Sandbox, Settings, Text,
};

thread_local! {
    static START_TIME: RefCell<Option<Instant>> = RefCell::new(None);
}
pub fn main() -> iced::Result {
    START_TIME.with(|t| *t.borrow_mut() = Some(Instant::now()));
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        START_TIME.with(|t| {
            let mut t = t.borrow_mut();
            if let Some(start_time) = *t {
                println!("first paint {:?}", start_time.elapsed());
                *t = None;
            }
        });
        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}
