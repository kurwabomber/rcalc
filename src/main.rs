use iced::widget::text_input::Id;
use iced::widget::{button, column, text, text_input};
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Calculator::run(Settings::default())
}

struct Calculator {
    formula: CalculatorInput,//will become formula(s)
}

struct CalculatorInput{
    input: String,
    text_input_id: Id,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String)
}

impl Sandbox for Calculator {
    type Message = Message;

    fn new() -> Self {
        Self {
            formula: CalculatorInput{ 
                input: "".to_string(),
                text_input_id: Id::new(format!("formula-1")) 
            },
        }
    }

    fn title(&self) -> String {
        String::from("rcalc")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(new_formula) => {
                println!("changed the function to -> {}", new_formula);
                self.formula.input = new_formula;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text_input(&self.formula.input, &self.formula.input)
                .on_input(Message::InputChanged)
                .id(self.formula.text_input_id.clone())
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}