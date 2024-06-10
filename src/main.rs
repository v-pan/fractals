use glam::{vec3, Vec3};
use iced::{alignment, executor, Length};
use iced::{Application, Command, Element, Settings, Theme};

use iced::widget::{button, column, container, image, row, text, text_input};

const IMG_H: usize = 750;
const IMG_W: usize = 750;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Clone)]
struct ImageParameters {
    image_width: usize,
    image_height: usize,
    camera_position: Vec3,
    camera_direction: Vec3,
    max_steps: i16,
    min_distance: f32,    
}

impl Default for ImageParameters {
    fn default() -> Self {
        ImageParameters {
            image_height: IMG_H,
            image_width: IMG_W,
            camera_position: vec3(0.0, 0.0, 5.0),
            camera_direction: vec3(0.0, 0.0, -1.0),
            max_steps: 100,
            min_distance: 0.00001
        }
    }
}

struct App {
    state: State
}

#[derive(Default, Clone)]
struct State {
    params: ImageParameters,
    pixels: Vec<u8>
}

#[derive(Debug, Clone)]
enum Message {
    RenderImage,
    
    ImageWidthChanged(String),
    ImageHeightChanged(String),
    
    CameraPositionXChanged(String),
    CameraPositionYChanged(String),
    CameraPositionZChanged(String),

    CameraDirectionXChanged(String),
    CameraDirectionYChanged(String),
    CameraDirectionZChanged(String),

    MaxStepsChanged(String),
    MinDistanceChanged(String),
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        (App { 
            state: State {
                pixels: vec![0; 4 * IMG_W * IMG_H],
                ..State::default()
            } 
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Fractal renderer")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::RenderImage => {
                let pixels = trace_image(self.state.params.clone());

                *self = App {
                    state: State {
                        pixels,
                        ..self.state.clone()
                    }
                };
            },

            Message::ImageWidthChanged(new) => {
                self.state.params.image_width = new.parse().unwrap_or(self.state.params.image_width);
            },
            Message::ImageHeightChanged(new) => {
                self.state.params.image_height = new.parse().unwrap_or(self.state.params.image_height);
            },

            Message::CameraPositionXChanged(new) => {
                self.state.params.camera_position.x = new.parse().unwrap_or(self.state.params.camera_position.x);
            },
            Message::CameraPositionYChanged(new) => {
                self.state.params.camera_position.y = new.parse().unwrap_or(self.state.params.camera_position.y);
            },
            Message::CameraPositionZChanged(new) => {
                self.state.params.camera_position.z = new.parse().unwrap_or(self.state.params.camera_position.z);
            },

            Message::CameraDirectionXChanged(new) => {
                self.state.params.camera_direction.x = new.parse().unwrap_or(self.state.params.camera_direction.x);
            },
            Message::CameraDirectionYChanged(new) => {
                self.state.params.camera_direction.y = new.parse().unwrap_or(self.state.params.camera_direction.y);
            },
            Message::CameraDirectionZChanged(new) => {
                self.state.params.camera_direction.z = new.parse().unwrap_or(self.state.params.camera_direction.z);
            },

            Message::MaxStepsChanged(new) => {
                self.state.params.max_steps = new.parse().unwrap_or(self.state.params.max_steps);
            },
            Message::MinDistanceChanged(new) => {
                self.state.params.min_distance = new.parse().unwrap_or(self.state.params.min_distance);
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let image_width = self.state.params.image_width;
        let image_height = self.state.params.image_height;

        let handle = image::Handle::from_pixels(
            image_width as u32,
            image_height as u32,
            self.state.pixels.clone()
        );

        let ImageParameters {
            image_height,
            image_width,
            camera_direction,
            camera_position,
            max_steps,
            min_distance
        } = self.state.params;

        column![
            row![
                text("Image Dimensions"),
                text_input("Width", image_width.to_string().as_str()).on_input(|new| { Message::ImageWidthChanged(new) }),
                text_input("Height", image_height.to_string().as_str()).on_input(|new| { Message::ImageHeightChanged(new) }),
            ].spacing(20).padding(5),
            row![
                text("Max steps"),
                text_input("Max steps", max_steps.to_string().as_str()).on_input(|new| { Message::MaxStepsChanged(new) }),
                text("Min distance"),
                text_input("Min distance", min_distance.to_string().as_str()).on_input(|new| { Message::MinDistanceChanged(new) }),
            ].spacing(20).padding(5),
            row![
                text("Camera Position"),
                text_input("X", camera_position.x.to_string().as_str()).on_input(|new| { Message::CameraPositionXChanged(new) }),
                text_input("Y", camera_position.y.to_string().as_str()).on_input(|new| { Message::CameraPositionYChanged(new) }),
                text_input("Z", camera_position.z.to_string().as_str()).on_input(|new| { Message::CameraPositionZChanged(new) }),
            ].spacing(20).padding(5),
            row![
                text("Camera Direction"),
                text_input("X", camera_direction.x.to_string().as_str()).on_input(|new| { Message::CameraDirectionXChanged(new) }),
                text_input("Y", camera_direction.y.to_string().as_str()).on_input(|new| { Message::CameraDirectionYChanged(new) }),
                text_input("Z", camera_direction.z.to_string().as_str()).on_input(|new| { Message::CameraDirectionZChanged(new) }),
            ].spacing(20).padding(5),
            button("Render").on_press(Message::RenderImage),
            image::viewer(handle),
        ]
        .width(Length::Fill)
        .into()
    }
}

const MAX_STEPS: i16 = 100;
const MIN_DISTANCE: f32 = 0.0001;

fn trace_image(image_params: ImageParameters) -> Vec<u8> {
    let image_width = image_params.image_width;
    let image_height = image_params.image_height;
    let camera_position = image_params.camera_position;

    // For now, "texture" grid will be 1units x 1units, "located" 1unit in front of the camera
    let camera_direction = vec3(0.0, 0.0, -1.0);

    let mut buffer: Vec<u8> = Vec::with_capacity(4 * image_width * image_height);

    // Divide up the grid into rays
    for i in 0..image_width {
        for j in 0..image_height {
            let direction = camera_direction + vec3(
                (i as i32 - image_width as i32/2) as f32 / image_width as f32,
                (j as i32 - image_height as i32/2) as f32 / image_height as f32,
                0.0,
            );

            // Trace the ray, compute a colour, store in buffer
            let result = trace(camera_position, direction, image_params.max_steps, image_params.min_distance);

            buffer.push((result * 255.0) as u8);
            buffer.push((result * 255.0) as u8);
            buffer.push((result * 255.0) as u8);
            buffer.push(255);
        }
    }

    return buffer;
}

fn trace(from: Vec3, direction: Vec3, max_steps: i16, min_distance: f32) -> f32{
    let mut total_distance: f32 = 0.0;
    
    for step in 0..max_steps {
        let current_point: Vec3 = from + (total_distance * direction);
        let distance = sdf(current_point);
        total_distance += distance;

        if distance < min_distance {
            return 1.0 - Into::<f32>::into(step) / Into::<f32>::into(max_steps);
        }
    }

    return 0.0;
}

fn sdf(point: Vec3) -> f32 {
    let x = point.x % 1.0;
    let y = point.y % 1.0;

    let instance = vec3(x, y, point.z) - vec3(0.5, 0.5, 0.5);

    return instance.length() - 0.4;
}