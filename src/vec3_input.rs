// use iced::widget::{row, text, text_input};
//
// #[derive(Debug, Clone)]
// pub enum Message {
//     ChangeInput1(String),
//     ChangeInput2(String),
//     ChangeInput3(String),
// }
//
// pub enum Action {
//     Change(f32, f32, f32),
//     None,
// }
//
// #[derive(Default, Clone)]
// pub struct Vec3Input {
//     title: String,
//     input1_title: String,
//     input2_title: String,
//     input3_title: String,
//     input1_content: String,
//     input2_content: String,
//     input3_content: String,
// }
//
// impl Vec3Input {
//     pub fn new() -> Self {
//         Self {
//             title: "".to_owned(),
//             input1_title: "".to_owned(),
//             input2_title: "".to_owned(),
//             input3_title: "".to_owned(),
//             input1_content: "0.0".to_string(),
//             input2_content: "0.0".to_string(),
//             input3_content: "0.0".to_string(),
//         }
//     }
//
//     pub fn title(self, title: &str) -> Self {
//         Self {
//             title: title.to_owned(),
//             ..self.clone()
//         }
//     }
//
//     pub fn placeholders(self, input1: &str, input2: &str, input3: &str) -> Self {
//         Self {
//             input1_title: input1.to_owned(),
//             input2_title: input2.to_owned(),
//             input3_title: input3.to_owned(),
//             ..self.clone()
//         }
//     }
//
//     pub fn values(self, input1: &str, input2: &str, input3: &str) -> Self {
//         Self {
//             input1_content: input1.to_string(),
//             input2_content: input2.to_string(),
//             input3_content: input3.to_string(),
//             ..self.clone()
//         }
//     }
// }
//
// impl<'a, Message> From<Vec3Input> for iced::Element<'a, Message> {
//     fn from(value: Vec3Input) -> Self {
//         row![
//             text(value.title),
//             text_input(&value.input1_title, &value.input1_content)
//                 .on_input(|new| { Message::ChangeInput1(new) }),
//             text_input(&value.input1_title, &value.input2_content)
//                 .on_input(|new| { Message::ChangeInput2(new) }),
//             text_input(&value.input1_title, &value.input3_content)
//                 .on_input(|new| { Message::ChangeInput3(new) }),
//         ]
//         .spacing(20)
//         .padding(5)
//         .into()
//     }
// }
//
// impl Vec3Input {
//     pub fn view(&self) -> iced::Element<'_, Message> {
//         row![
//             text(&self.title),
//             text_input(&self.input1_title, &self.input1_content)
//                 .on_input(|new| { Message::ChangeInput1(new) }),
//             text_input(&self.input1_title, &self.input2_content)
//                 .on_input(|new| { Message::ChangeInput2(new) }),
//             text_input(&self.input1_title, &self.input3_content)
//                 .on_input(|new| { Message::ChangeInput3(new) }),
//         ]
//         .spacing(20)
//         .padding(5)
//         .into()
//     }
// }
//
// impl Vec3Input {
//     #[must_use]
//     pub fn update(&mut self, message: Message) -> Action {
//         match message {
//             Message::ChangeInput1(new) => {
//                 let parsed = new.parse::<f32>();
//
//                 match parsed {
//                     Ok(_) => {
//                         self.input1_content = new;
//                         Action::Change(
//                             self.input1_content.parse().unwrap(),
//                             self.input2_content.parse().unwrap(),
//                             self.input3_content.parse().unwrap(),
//                         )
//                     }
//                     _ => Action::None,
//                 }
//             }
//             Message::ChangeInput2(new) => {
//                 let parsed = new.parse::<f32>();
//
//                 match parsed {
//                     Ok(_) => {
//                         self.input2_content = new;
//                         Action::Change(
//                             self.input1_content.parse().unwrap(),
//                             self.input2_content.parse().unwrap(),
//                             self.input3_content.parse().unwrap(),
//                         )
//                     }
//                     _ => Action::None,
//                 }
//             }
//             Message::ChangeInput3(new) => {
//                 let parsed = new.parse::<f32>();
//
//                 match parsed {
//                     Ok(_) => {
//                         self.input3_content = new;
//                         Action::Change(
//                             self.input1_content.parse().unwrap(),
//                             self.input2_content.parse().unwrap(),
//                             self.input3_content.parse().unwrap(),
//                         )
//                     }
//                     _ => Action::None,
//                 }
//             }
//         }
//     }
// }
