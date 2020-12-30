// Copyright 2020-2021 Benoit Walter

use iced::{
    button, scrollable, Application, Button, Column, Command, Element, HorizontalAlignment,
    Scrollable, Settings, Text,
};

mod diagram;
mod parser;

//use crate::canvas::Canvas;
use crate::diagram::Diagram;

pub fn main() -> iced::Result {
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");

    Vuk::run(Settings::default())
}

struct Vuk {
    items: Vec<diagram::model::Item>,

    button_add_state: button::State,
    scrollable_state: scrollable::State,
}

#[derive(Clone, Debug)]
enum Message {
    Add,
    ItemMessage(usize, diagram::item::Message),
}

impl Application for Vuk {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Vuk {
                items: Vec::new(),
                button_add_state: button::State::new(),
                scrollable_state: scrollable::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Vuk".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Add => {
                self.items.push(diagram::model::Item::new(
                    diagram::model::ItemType::Interface,
                    "My Interface",
                ));
            }
            Message::ItemMessage(_, _) => {}
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let button_add = Button::new(
            &mut self.button_add_state,
            Text::new("Add").horizontal_alignment(HorizontalAlignment::Center),
        )
        .on_press(Message::Add);

        let items: Element<_> = self
            .items
            .iter_mut()
            .enumerate()
            .fold(Diagram::new(), |sheet, (i, item)| {
                sheet.push(
                    diagram::Item::new(item.clone()), //.map(move |message| Message::ItemMessage(i, message)),
                )
            })
            .into();

        let root = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(button_add)
            .push(Scrollable::new(&mut self.scrollable_state).push(items));

        root.into()
    }
}
