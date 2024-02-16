// use iced::{button, Button, Column, Command, Element, Sandbox, Settings, Text};

// #[derive(Default)]
// pub struct MyApp {
//     button_state1: button::State,
//     button_state2: button::State,
// }

// #[derive(Debug, Clone)]
// pub enum MyMessage {
//     Button1Clicked,
//     Button2Clicked,
// }

// impl Sandbox for MyApp {
//     type Message = MyMessage;

//     fn new() -> Self {
//         Self::default()
//     }

//     fn title(&self) -> String {
//         String::from("My App")
//     }

//     fn update(&mut self, message: Self::Message) {
//         match message {
//             MyMessage::Button1Clicked => {
//                 println!("Button 1 clicked");
//             }
//             MyMessage::Button2Clicked => {
//                 // Do nothing
//             }
//         }
//     }

//     fn view(&mut self) -> Element<Self::Message> {
//         Column::new()
//             .push(Button::new(&mut self.button_state1, Text::new("Button 1")).on_press(MyMessage::Button1Clicked))
//             .push(Button::new(&mut self.button_state2, Text::new("Button 2")).on_press(MyMessage::Button2Clicked))
//             .into()
//     }
// }

// fn main() -> iced::Result {
//     // MyApp::run(Settings::default())
// }

mod game;
use game::terminal_game;


fn main() {
    terminal_game();
}
