use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};

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
        "Fractal goes here".into()
    }
}