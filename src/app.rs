use iced::Element;
use iced::widget::button;
use iced::widget::column;
use iced_plot::PlotUiMessage;

use crate::input_signal_plot::InputSignalPlot;

pub struct App {
    input_signal_plot: InputSignalPlot,
}

impl App {
    fn new() -> Self {
        Self {
            input_signal_plot: InputSignalPlot::new(),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    AddMorePoints,
    PlotMessage(PlotUiMessage),
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::AddMorePoints => {
                self.input_signal_plot.add_more_points();
            }
            Message::PlotMessage(plot_ui_message) => self.input_signal_plot.update(plot_ui_message),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let plot_widget = self.input_signal_plot.view().map(Message::PlotMessage);

        column![
            button("More!").on_press(Message::AddMorePoints),
            plot_widget,
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}
