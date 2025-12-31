use iced::{event::Status, widget::shader};

use crate::{app::Message, shader::primitive::ShaderPrimitive};

#[derive(Default)]
pub enum State {
    #[default]
    Idle,
}

pub struct ShaderProgram {}

impl ShaderProgram {
    pub fn new() -> Self {
        Self {}
    }
}

impl shader::Program<Message> for ShaderProgram {
    type State = State;
    type Primitive = ShaderPrimitive;

    fn draw(
        &self,
        state: &Self::State,
        cursor: iced::advanced::mouse::Cursor,
        bounds: iced::Rectangle,
    ) -> Self::Primitive {
        Self::Primitive::new()
    }

    fn update(
        &self,
        _state: &mut Self::State,
        _event: shader::Event,
        _bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
        _shell: &mut iced::advanced::Shell<'_, Message>,
    ) -> (Status, Option<Message>) {
        (Status::Ignored, None)
    }
}
