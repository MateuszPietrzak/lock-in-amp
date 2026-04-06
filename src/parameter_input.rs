use iced::Element;
use iced::widget::button;
use iced::widget::column;
use iced::widget::row;
use iced::widget::text;
use iced::widget::text_input;

pub struct ParameterInput {
    content: String,
    label: String
}

#[derive(Debug, Clone)]
pub enum ParameterInputMessage {
    ContentChanged(String),
    Submit,
}

impl ParameterInput {
    pub fn new(label: String, default_value: f32) -> Self {
        Self {
            label: label,
            content: default_value.to_string()
        }
    }

    pub fn update(&mut self, message: ParameterInputMessage) -> Option<f32> {
        match message {
            ParameterInputMessage::ContentChanged(value) => {
                if value.is_empty() || value.parse::<f32>().is_ok() {
                    self.content = value;
                }
                None
            }
            ParameterInputMessage::Submit => {
                self.content.parse::<f32>().ok()
            }
        }
    }

    pub fn view(&'_ self) -> Element<'_, ParameterInputMessage> {
        column![
            text(&self.label),
            row![
                text_input(&self.content, &self.content)
                    .on_input(ParameterInputMessage::ContentChanged)
                    .width(150)
                    .padding(10),
                button("Set").on_press(ParameterInputMessage::Submit),
            ]
            .align_y(iced::alignment::Vertical::Center)
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}
