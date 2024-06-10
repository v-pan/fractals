use glam::{vec3, Vec3};
use iced::{executor, Length};
use iced::{Application, Command, Element, Settings, Theme};

use iced::widget::{button, column, image, row, text, text_input};

const DEFAULT_IMG_H: usize = 750;
const DEFAULT_IMG_W: usize = 1000;
const DEFAULT_CAMERA_POSITION: Vec3 = vec3(0.0, 0.0, 3.0);
const DEFAULT_CAMERA_DIRECTION: Vec3 = vec3(0.0, 0.0, -1.0);
const DEFAULT_MAX_STEPS: i16 = 1000;
const DEFAULT_MIN_DISTANCE: f32 = 0.001;
const DEFAULT_MAX_DISTANCE: f32 = 1000.0;

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
            image_height: DEFAULT_IMG_H,
            image_width: DEFAULT_IMG_W,
            camera_position: DEFAULT_CAMERA_POSITION,
            camera_direction: DEFAULT_CAMERA_DIRECTION,
            max_steps: DEFAULT_MAX_STEPS,
            min_distance: DEFAULT_MIN_DISTANCE,
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
                pixels: vec![0; 4 * DEFAULT_IMG_W * DEFAULT_IMG_H],
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

fn trace_image(image_params: ImageParameters) -> Vec<u8> {
    let ImageParameters {
        image_width,
        image_height,
        camera_position,
        ..
    } = image_params;

    let aspect_ratio: f32 = image_width as f32 / image_height as f32;

    // For now, "texture" grid will be 1units x 1units, "located" 1unit in front of the camera
    let camera_direction = DEFAULT_CAMERA_DIRECTION;

    let mut buffer: Vec<u8> = Vec::with_capacity(4 * image_width * image_height);

    // Divide up the grid into rays
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let x: f32 = ((i as f32 / image_width as f32) - 0.5) * aspect_ratio;
            let y: f32 = (j as f32 / image_height as f32) - 0.5;

            let direction = camera_direction + vec3(x, y, 0.0);

            // Trace the ray, compute a colour, store in buffer
            let result = trace(camera_position, direction, image_params.max_steps, image_params.min_distance);

            buffer.push((result * 255.0) as u8);
            buffer.push((result * 255.0) as u8);
            buffer.push((result * 255.0) as u8);
            buffer.push(255);

            // Draw UV coords
            // buffer.push((x * 255.0) as u8);
            // buffer.push((y * 255.0) as u8);
            // buffer.push(0);
            // buffer.push(255);
        }
    }

    return buffer;
}

fn trace(from: Vec3, direction: Vec3, max_steps: i16, min_distance: f32) -> f32{
    let mut total_distance: f32 = 0.0;

    for step in 0..max_steps {
        let current_point: Vec3 = from + (total_distance * direction);
        let distance = sierpinsky_sdf(current_point);

        if distance > DEFAULT_MAX_DISTANCE { break; }

        total_distance += distance;

        if distance < min_distance {
            return 1.0 - Into::<f32>::into(step) / Into::<f32>::into(max_steps);
        }
    }

    return 0.0;
}

fn sierpinsky_sdf(point: Vec3) -> f32 {
    let max_iterations = 10;
    let scale = 2.0;

    let mut point = point;

    let a1: Vec3 = vec3(1.0,1.0,1.0);
	let a2: Vec3 = vec3(-1.0,-1.0,1.0);
	let a3: Vec3 = vec3(1.0,-1.0,-1.0);
	let a4: Vec3 = vec3(-1.0,1.0,-1.0);
	let mut c: Vec3;

    let mut d = 0.0;
    let mut dist = 0.0;

    for step in 0..max_iterations {
        c = a1;
        dist = (point - a1).length();
        
        d = (point - a2).length();
        if d < dist {
            c = a2;
            dist = d;
        }

        d = (point - a3).length();
        if d < dist {
            c = a3;
            dist = d;
        }

        d = (point - a4).length();
        if d < dist {
            c = a4;
            dist = d;
        }

        point = scale * point - c*(scale - 1.0);
    }

    return point.length() * scale.powf(-max_iterations as f32);
}

fn spheres_sdf(point: Vec3) -> f32 {
    let x = point.x.signum() * (point.x % 1.0);
    let y = point.y.signum() * (point.y % 1.0);

    let instance = vec3(x, y, point.z) - vec3(0.5, 0.5, 0.5);

    return instance.length() - 0.4;
}