use iced::{text_input, Color, Column, Element, Sandbox, Settings, Text, TextInput};

#[derive(Default)]
struct State {
    input: text_input::State,
    input_value: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
}

impl Sandbox for State {
    type Message = Message;

    fn new() -> State {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Character Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(new_val) => {
                self.input_value = new_val;
            }
        }
    }

    fn background_color(&self) -> Color {
        Color::new(0.5, 0.5, 0.5, 0.5)
    }

    fn view(&mut self) -> Element<Message> {
        let text = if self.input_value.is_empty() {
            String::from("Please enter something in the input box above")
        } else {
            format!(
                "{} contains {} characters.",
                self.input_value,
                self.input_value.len()
            )
        };

        Column::new()
            .push(Text::new("What is the input string"))
            .push(
                TextInput::new(
                    &mut self.input,
                    "Input String",
                    &self.input_value,
                    Message::InputChanged,
                )
                .padding(15)
                .size(30),
            )
            .push(Text::new(&text))
            .into()
    }
}

fn main() -> iced::Result {
    State::run(Settings::default())
}
