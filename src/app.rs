use iced::Element;
use iced::widget::button;
use iced::widget::row;
use iced_plot::PlotUiMessage;

use crate::input_signal_plot::InputSignalPlot;
use crate::input_signal_settings::{InputSignalSettingsWidget, InputSignalSettings, InputSignalSettingsMessage};

pub struct App {
    input_signal_plot: InputSignalPlot,
    input_signal_settings_widget: InputSignalSettingsWidget,
}

impl App {
    fn new() -> Self {
        Self {
            input_signal_plot: InputSignalPlot::new(),
            input_signal_settings_widget: InputSignalSettingsWidget::new(InputSignalSettings {
                len_sec: 1.0,
                fs: 500.0,
                noise_ampl: 10.0,
                sig_ampl: 1.0,
                sig_freq: 10.0,
                sig_phase_deg: 90.0,
            })
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    AddMorePoints,
    PlotMessage(PlotUiMessage),
    InputParameterChangedMessage(InputSignalSettingsMessage),
}

impl App {
    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::AddMorePoints => self.input_signal_plot.add_more_points(),
            AppMessage::PlotMessage(plot_ui_message) => self.input_signal_plot.update(plot_ui_message),
            AppMessage::InputParameterChangedMessage(param_changed_message) => {
                    
                if let Some(new_settings) = self.input_signal_settings_widget.update(param_changed_message) {
                    println!("new settings received: {:?}", new_settings);
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        let plot_widget = self.input_signal_plot.view().map(AppMessage::PlotMessage);
        let input_settings_widget = self.input_signal_settings_widget.view().map(AppMessage::InputParameterChangedMessage);
        row![
            input_settings_widget,
            button("More!").on_press(AppMessage::AddMorePoints),
            plot_widget,
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}
