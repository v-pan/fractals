use iced::executor;
use iced::widget::image::{self, Handle};
use iced::widget::Image;
use iced::{Application, Command, Element, Settings, Theme};

const IMG_H: usize = 100;
const IMG_W: usize = 100;
const IMG_TEST1: [u8; 4 * IMG_H * IMG_W] = [100; 4 * IMG_H * IMG_W];

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

struct App;

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        (App, Command::none())
    }

    fn title(&self) -> String {
        String::from("Fractal renderer")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let handle = image::Handle::from_pixels(
            IMG_W as u32,
            IMG_H as u32,
            IMG_TEST1
        );

        image::viewer(handle).into()
    }
}