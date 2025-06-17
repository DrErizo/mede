use iced::widget::{button, column, text, Column};
use iced::{Alignment,Length};
use iced::Theme;

pub fn main() -> iced::Result {
    iced::application("Mede", update, view)
        .theme(|_| Theme::Dark)
        .centered()
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

fn update(value: &mut u64, message: Message) {
    match message {
        Message::Increment => *value += 1,
    }
}

fn view(value: &u64) -> Column<Message> {
    column![
        text(value),
        button("+").on_press(Message::Increment),
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Center)
}
