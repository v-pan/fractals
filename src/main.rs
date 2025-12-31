use iced::{Application, Settings};

mod app;
mod camera;
mod scene;
mod sdf;
mod shader;
mod vec3_input;

pub fn main() -> iced::Result {
    app::App::run(Settings::default())
}
