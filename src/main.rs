use glam::{vec3, Vec3};
use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};

use iced::widget::image;

const IMG_H: usize = 500;
const IMG_W: usize = 1000;
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
        let pixels = trace_image(
            vec3(2.0, 0.0, 0.0),
        );

        let handle = image::Handle::from_pixels(
            IMG_W as u32,
            IMG_H as u32,
            pixels
        );

        image::viewer(handle).into()
    }
}

const MAX_STEPS: i16 = 20;
const MIN_DISTANCE: f32 = 0.0001;

fn trace_image(camera_position: Vec3) -> Vec<u8> {
    // For now, "texture" grid will be 1units x 1units, "located" 1unit in front of the camera
    let camera_direction = vec3(-1.0, 0.0, 0.0);

    let mut buffer: Vec<u8> = Vec::with_capacity(4 * IMG_W * IMG_H);
    
    // Divide up the grid into rays
    for i in 0..IMG_W {
        for j in 0..IMG_H {
            let direction = camera_direction + vec3(
                0.0,
                (i as i32 - IMG_W as i32/2) as f32 / IMG_W as f32,
                (j as i32 - IMG_H as i32/2) as f32 / IMG_H as f32
            );

            // Trace the ray, compute a colour, store in buffer
            let result = trace(camera_position, direction);

            buffer.push((result * 255.0) as u8);
            buffer.push((result * 255.0) as u8);
            buffer.push((result * 255.0) as u8);
            buffer.push(255);
        }
    }

    return buffer;
}

fn trace(from: Vec3, direction: Vec3) -> f32{
    let mut total_distance: f32 = 0.0;
    
    for step in 0..MAX_STEPS {
        let current_point: Vec3 = from + (total_distance * direction);
        let distance = sdf(current_point);
        total_distance += distance;

        if distance < MIN_DISTANCE {
            return 1.0 - Into::<f32>::into(step) / Into::<f32>::into(MAX_STEPS);
        }
    }

    return 0.0;
}

fn sdf(point: Vec3) -> f32 {
    return point.length() - 0.5;
}