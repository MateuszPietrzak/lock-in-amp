use iced::Element;
use iced::widget::column;
use iced::widget::text;

use crate::parameter_input::*;

#[derive(Debug, Clone, Copy)]
pub struct InputSignalSettings {
    pub len_sec: f32,
    pub fs: f32,
    pub noise_ampl: f32,
    pub sig_ampl: f32,
    pub sig_freq: f32,
    pub sig_phase_deg: f32,
}

pub struct InputSignalSettingsWidget {
    settings: InputSignalSettings,

    len_sec_input: ParameterInput,
    fs_input: ParameterInput,
    noise_ampl_input: ParameterInput,
    sig_ampl_input: ParameterInput,
    sig_freq_input: ParameterInput,
    sig_phase_deg_input: ParameterInput,
}

#[derive(Debug, Clone)]
pub enum InputSignalSettingsMessage {
    LenSecChanged(ParameterInputMessage),
    FsChanged(ParameterInputMessage),
    NoiseAmplChanged(ParameterInputMessage),
    SigAmplChanged(ParameterInputMessage),
    SigFreqChanged(ParameterInputMessage),
    SigPhaseChanged(ParameterInputMessage),
}

impl InputSignalSettingsWidget {
    pub fn new(default_settings: InputSignalSettings) -> Self {
        Self {
            settings: default_settings.clone(),

            len_sec_input: ParameterInput::new(
                String::from("Length [s]"),
                default_settings.len_sec,
            ),
            fs_input: ParameterInput::new(
                String::from("Sampling frequency [Hz]"),
                default_settings.fs,
            ),
            noise_ampl_input: ParameterInput::new(
                String::from("Noise amplitude [-]"),
                default_settings.noise_ampl,
            ),
            sig_ampl_input: ParameterInput::new(
                String::from("Signal amplitude [-]"),
                default_settings.sig_ampl,
            ),
            sig_freq_input: ParameterInput::new(
                String::from("Signal frequency [Hz]"),
                default_settings.sig_freq,
            ),
            sig_phase_deg_input: ParameterInput::new(
                String::from("Signal phase [deg]"),
                default_settings.sig_phase_deg,
            ),
        }
    }

    pub fn update(&mut self, message: InputSignalSettingsMessage) -> Option<InputSignalSettings> {
        match message {
            InputSignalSettingsMessage::LenSecChanged(new_value) => {
                if let Some(val) = self.len_sec_input.update(new_value) {
                    self.settings.len_sec = val;
                    return Some(self.settings.clone());
                }
                None
            }
            InputSignalSettingsMessage::FsChanged(new_value) => {
                if let Some(val) = self.fs_input.update(new_value) {
                    self.settings.fs = val;
                    return Some(self.settings.clone());
                }
                None
            }
            InputSignalSettingsMessage::NoiseAmplChanged(new_value) => {
                if let Some(val) = self.noise_ampl_input.update(new_value) {
                    self.settings.noise_ampl = val;
                    return Some(self.settings.clone());
                }
                None
            }
            InputSignalSettingsMessage::SigAmplChanged(new_value) => {
                if let Some(val) = self.sig_ampl_input.update(new_value) {
                    self.settings.sig_ampl = val;
                    return Some(self.settings.clone());
                }
                None
            }
            InputSignalSettingsMessage::SigFreqChanged(new_value) => {
                if let Some(val) = self.sig_freq_input.update(new_value) {
                    self.settings.sig_freq = val;
                    return Some(self.settings.clone());
                }
                None
            }
            InputSignalSettingsMessage::SigPhaseChanged(new_value) => {
                if let Some(val) = self.sig_phase_deg_input.update(new_value) {
                    self.settings.sig_phase_deg = val;
                    return Some(self.settings.clone());
                }
                None
            }
        }
    }

    pub fn view(&'_ self) -> Element<'_, InputSignalSettingsMessage> {
        let len_sec_input = self
            .len_sec_input
            .view()
            .map(InputSignalSettingsMessage::LenSecChanged);
        let fs_input = self
            .fs_input
            .view()
            .map(InputSignalSettingsMessage::FsChanged);
        let noise_ampl_input = self
            .noise_ampl_input
            .view()
            .map(InputSignalSettingsMessage::NoiseAmplChanged);
        let sig_ampl_input = self
            .sig_ampl_input
            .view()
            .map(InputSignalSettingsMessage::SigAmplChanged);
        let sig_freq_input = self
            .sig_freq_input
            .view()
            .map(InputSignalSettingsMessage::SigFreqChanged);
        let sig_phase_deg_input = self
            .sig_phase_deg_input
            .view()
            .map(InputSignalSettingsMessage::SigPhaseChanged);

        column![
            text("INPUT SIGNAL SETTINGS").size(20),
            len_sec_input,
            fs_input,
            noise_ampl_input,
            sig_ampl_input,
            sig_freq_input,
            sig_phase_deg_input
        ]
        .into()
    }
}
