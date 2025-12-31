use iced::{widget::column, Length, Sandbox};

use crate::shader::program::ShaderProgram;

#[derive(Debug)]
pub enum Message {}

pub struct App {
    program: ShaderProgram,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            program: ShaderProgram::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Fractal Renderer")
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::shader(&self.program)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn update(&mut self, message: Self::Message) {}
}
