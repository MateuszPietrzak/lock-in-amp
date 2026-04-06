use iced::Element;
use iced::widget::{button, text};
use iced::widget::{row, column};
use iced_plot::PlotUiMessage;
use std::sync::Arc;

use crate::input_signal_gen::generate_input_time_axis;
use crate::signal_plot::SignalPlot;
use crate::input_signal_settings::{InputSignalSettingsWidget, InputSignalSettings, InputSignalSettingsMessage};
use crate::input_signal_gen::generate_input_signal;
use crate::lock_in_settings::{LockInSettings, LockInSettingsWidget, LockInSettingsMessage};
use crate::lock_in_amplifier::analyze;

pub struct App {
    // widgets
    input_signal_plot: SignalPlot,
    input_signal_settings_widget: InputSignalSettingsWidget,
    lock_in_settings_widget: LockInSettingsWidget,
    measured_ampl_plot_widget: SignalPlot,
    measured_phase_plot_widget: SignalPlot,

    // states
    input_signal: Arc<Vec<f32>>,
    input_time_axis: Arc<Vec<f32>>,
    output_ampl: Arc<Vec<f32>>,
    output_phase: Arc<Vec<f32>>,

    lockin_settings: LockInSettings
}

impl App {
    fn new() -> Self {

        let default_input_settings = InputSignalSettings {
                len_sec: 100.0,
                fs: 1000.0,
                noise_ampl: 0.0,
                sig_ampl: 1.0,
                sig_freq: 10.0,
                sig_phase_deg: 90.0,
            };
        let default_lockin_settings = LockInSettings {
            fs: 1000.0,
            ref_freq: 10.0,
            lpf_tau: 0.1,
            lpf_iir_order: 4,
        };

        let input_signal = generate_input_signal(default_input_settings);
        let input_time_axis = generate_input_time_axis(default_input_settings);

        Self {
            input_signal_plot: SignalPlot::new(),
            input_signal_settings_widget: InputSignalSettingsWidget::new(default_input_settings),
            lock_in_settings_widget: LockInSettingsWidget::new(default_lockin_settings.clone()),
            input_signal: Arc::new(input_signal),
            input_time_axis: Arc::new(input_time_axis),
            measured_ampl_plot_widget: SignalPlot::new(),
            measured_phase_plot_widget: SignalPlot::new(),
            lockin_settings: default_lockin_settings,
            output_ampl: Arc::new(Vec::<f32>::new()),
            output_phase: Arc::new(Vec::<f32>::new()),
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
    InputPlotMessage(PlotUiMessage),
    InputParameterChangedMessage(InputSignalSettingsMessage),
    LockinParamChangedMessage(LockInSettingsMessage),
    MeasAmplPlotMessage(PlotUiMessage),
    MeasPhasePlotMessage(PlotUiMessage),
}

impl App {
    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::InputPlotMessage(plot_ui_message) => self.input_signal_plot.update(plot_ui_message),
            AppMessage::InputParameterChangedMessage(param_changed_message) => {
                    
                if let Some(new_settings) = self.input_signal_settings_widget.update(param_changed_message) {
                    println!("new input settings received: {:?}", new_settings);

                    // regenerate the signal
                    self.input_signal = Arc::new(generate_input_signal(new_settings));
                    self.input_time_axis = Arc::new(generate_input_time_axis(new_settings));

                    self.input_signal_plot.set_series(Arc::clone(&self.input_signal), Arc::clone(&self.input_time_axis));
                    
                    // we want the lock-in fs to be linked to input fs, so we manually set it here as it might've changed
                    self.lock_in_settings_widget.set_fs(new_settings.fs);   

                    self.reanalyze();
                }
            }
            AppMessage::LockinParamChangedMessage(param_changed_message) => {
                if let Some(new_settings) = self.lock_in_settings_widget.update(param_changed_message) {
                    println!("new lockin settings received: {:?}", new_settings);
                    self.lockin_settings = new_settings;

                    self.reanalyze();
                }
            }
            AppMessage::MeasAmplPlotMessage(plot_ui_message) => self.measured_ampl_plot_widget.update(plot_ui_message),
            AppMessage::MeasPhasePlotMessage(plot_ui_message) => self.measured_phase_plot_widget.update(plot_ui_message),

        }
    }

    fn reanalyze(&mut self) {
        let (meas_ampl, meas_phase) = analyze(Arc::clone(&self.input_signal), &self.lockin_settings);
        self.output_ampl = Arc::new(meas_ampl);
        self.output_phase = Arc::new(meas_phase);
        self.measured_ampl_plot_widget.set_series(Arc::clone(&self.output_ampl), Arc::clone(&self.input_time_axis));
        self.measured_phase_plot_widget.set_series(Arc::clone(&self.output_phase), Arc::clone(&self.input_time_axis));
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        let plot_widget = self.input_signal_plot.view().map(AppMessage::InputPlotMessage);
        let input_settings_widget = self.input_signal_settings_widget.view().map(AppMessage::InputParameterChangedMessage);
        let lock_in_settings_widget = self.lock_in_settings_widget.view().map(AppMessage::LockinParamChangedMessage);
        let measured_ampl_plot_widget = self.measured_ampl_plot_widget.view().map(AppMessage::MeasAmplPlotMessage);
        let measured_phase_plot_widget = self.measured_phase_plot_widget.view().map(AppMessage::MeasPhasePlotMessage);

        row![
            input_settings_widget,
            plot_widget,
            lock_in_settings_widget,
            column![
                text("MEASURED AMPLITUDE [-]"),
                measured_ampl_plot_widget,
                text("MEASURED PHASE [deg]"),
                measured_phase_plot_widget,
            ]
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}
