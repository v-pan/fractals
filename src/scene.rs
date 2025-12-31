use iced::widget::column;

#[derive(Debug, Clone)]
pub enum Message {
    Render,
    ChangeWidth(String),
    ChangeHeight(String),
}

pub enum Action {
    Render,
    ChangeImage,
    None,
}

pub struct Scene {}

impl Scene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene {
    pub fn view(&self) -> iced::Element<'_, Message> {
        column![].into()
    }
}

// impl Image {
//     #[must_use]
//     pub fn update(&mut self, message: Message) -> Action {
//         match message {
//             Message::Render => {
//                 // TODO: Render out a new image
//                 Action::None
//             }
//             Message::ChangeWidth(width) => {
//                 self.image_width = width.parse().unwrap_or(self.image_width);
//                 Action::None
//             }
//             Message::ChangeHeight(height) => {
//                 self.image_height = height.parse().unwrap_or(self.image_height);
//                 Action::None
//             }
//         }
//     }
// }
