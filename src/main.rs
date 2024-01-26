use std::ops::Add;

use iced::widget::text_input::Id;
use iced::widget::{button, row, text, text_input};
use iced::{Alignment, Element, Sandbox, Settings};
use evalexpr::*;

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
                self.formula.input = new_formula;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let expression = eval(&self.formula.input);
        let output_text: String;
        if expression.is_err(){
            output_text = expression.unwrap_err().to_string();
        }else{
            output_text = ["= ", &expression.unwrap().to_string()].concat()
        }

        row![
            text_input(&self.formula.input, &self.formula.input)
                .on_input(Message::InputChanged)
                .id(self.formula.text_input_id.clone())
                .width(600),
            text(&output_text)
        ]
        .padding(50)
        .spacing(100)
        .align_items(Alignment::Center)
        .into()
    }
}