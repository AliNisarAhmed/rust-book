use iced::{text_input, Color, Column, Element, Sandbox, Settings, Text, TextInput};

const INPUT_WIDTH: u32 = 500;

#[derive(Default)]
struct AppState {
    number_input_1: text_input::State,
    number_1: String,
    number_input_2: text_input::State,
    number_2: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    FirstNumberUpdated(String),
    SecondNumberUpdated(String),
}

impl Sandbox for AppState {
    type Message = Message;

    fn new() -> AppState {
        AppState::default()
    }

    fn title(&self) -> String {
        String::from("Simple Maths")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::FirstNumberUpdated(str) => {
                self.number_1 = str;
            }
            Message::SecondNumberUpdated(str) => {
                self.number_2 = str;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let input_1_error = input_error_text(&self.number_1);
        let input_2_error = input_error_text(&self.number_2);

        let input_number_1 = Column::new()
            .push(TextInput::new(
                &mut self.number_input_1,
                "First number",
                &self.number_1,
                |str| Message::FirstNumberUpdated(str),
            ))
            .push(input_1_error)
            .padding(10)
            .max_width(INPUT_WIDTH);

        let input_number_2 = Column::new()
            .push(TextInput::new(
                &mut self.number_input_2,
                "Second Number",
                &self.number_2,
                |str| Message::SecondNumberUpdated(str),
            ))
            .push(input_2_error)
            .padding(10)
            .max_width(INPUT_WIDTH);

        let stats = match (
            self.number_1.trim().parse::<isize>(),
            self.number_2.trim().parse::<isize>(),
        ) {
            (Ok(n1), Ok(n2)) => Column::new()
                .push(Text::new(format!("{} + {} = {}", n1, n2, n1 + n2)))
                .push(Text::new(format!("{} - {} = {}", n1, n2, n1 - n2)))
                .push(Text::new(format!("{} * {} = {}", n1, n2, n1 * n2)))
                .push(Text::new(format!("{} / {} = {}", n1, n2, n1 / n2))),
            _ => Column::new(),
        };

        Column::new()
            .push(input_number_1)
            .push(input_number_2)
            .push(stats)
            .into()
    }
}

fn input_error_text(s: &String) -> Text {
    Text::new(match s.trim().parse::<isize>() {
        Ok(_) => "",
        Err(_) => "Please enter a valid number",
    })
    .color(Color::new(1.0, 0.0, 0.0, 1.0))
}

fn main() -> iced::Result {
    AppState::run(Settings::default())
}
