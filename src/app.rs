use iced::Element;
use iced::widget::button;
use iced::widget::row;
use iced_plot::PlotUiMessage;
use std::sync::Arc;

use crate::input_signal_gen::generate_input_time_axis;
use crate::signal_plot::SignalPlot;
use crate::input_signal_settings::{InputSignalSettingsWidget, InputSignalSettings, InputSignalSettingsMessage};
use crate::input_signal_gen::generate_input_signal;
use crate::lock_in_settings::{LockInSettings, LockInSettingsWidget, LockInSettingsMessage};

pub struct App {
    // widgets
    input_signal_plot: SignalPlot,
    input_signal_settings_widget: InputSignalSettingsWidget,
    lock_in_settings_widget: LockInSettingsWidget,

    // states
    input_signal: Arc<Vec<f32>>,
    input_time_axis: Arc<Vec<f32>>,
}

impl App {
    fn new() -> Self {

        let default_input_settings = InputSignalSettings {
                len_sec: 1.0,
                fs: 500.0,
                noise_ampl: 10.0,
                sig_ampl: 1.0,
                sig_freq: 10.0,
                sig_phase_deg: 90.0,
            };
        let default_lockin_settings = LockInSettings {
            fs: 500.0,
            ref_freq: 1.0,
            lpf_tau: 0.1,
            lpf_iir_order: 4,
        };

        let input_signal = generate_input_signal(default_input_settings);
        let input_time_axis = generate_input_time_axis(default_input_settings);

        Self {
            input_signal_plot: SignalPlot::new(),
            input_signal_settings_widget: InputSignalSettingsWidget::new(default_input_settings),
            lock_in_settings_widget: LockInSettingsWidget::new(default_lockin_settings),
            input_signal: Arc::new(input_signal),
            input_time_axis: Arc::new(input_time_axis),
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
    PlotMessage(PlotUiMessage),
    InputParameterChangedMessage(InputSignalSettingsMessage),
    LockinParamChangedMessage(LockInSettingsMessage)
}

impl App {
    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::PlotMessage(plot_ui_message) => self.input_signal_plot.update(plot_ui_message),
            AppMessage::InputParameterChangedMessage(param_changed_message) => {
                    
                if let Some(new_settings) = self.input_signal_settings_widget.update(param_changed_message) {
                    println!("new input settings received: {:?}", new_settings);

                    // regenerate the signal
                    self.input_signal = Arc::new(generate_input_signal(new_settings));
                    self.input_time_axis = Arc::new(generate_input_time_axis(new_settings));

                    self.input_signal_plot.set_series(Arc::clone(&self.input_signal), Arc::clone(&self.input_time_axis))
                    // todo update lockin's settings with fs from new_settings!

                }
            }
            AppMessage::LockinParamChangedMessage(param_changed_message) => {
                if let Some(new_settings) = self.lock_in_settings_widget.update(param_changed_message) {
                    println!("new lockin settings received: {:?}", new_settings);
                    
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        let plot_widget = self.input_signal_plot.view().map(AppMessage::PlotMessage);
        let input_settings_widget = self.input_signal_settings_widget.view().map(AppMessage::InputParameterChangedMessage);
        let lock_in_settings_widget = self.lock_in_settings_widget.view().map(AppMessage::LockinParamChangedMessage);

        row![
            input_settings_widget,
            plot_widget,
            lock_in_settings_widget
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}
