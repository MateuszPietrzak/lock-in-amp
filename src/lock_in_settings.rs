use iced::Element;
use iced::widget::column;
use iced::widget::text;

use crate::parameter_input::*;

#[derive(Debug, Clone, Copy)]
pub struct LockInSettings {
    pub fs: f32,
    pub ref_freq: f32,
    pub lpf_tau: f32,
    pub lpf_iir_order: i32
}
pub struct LockInSettingsWidget {
    settings: LockInSettings,

    ref_freq_input: ParameterInput,
    lpf_tau_input: ParameterInput,
    lpf_iir_order_input: ParameterInput,
}

#[derive(Debug, Clone)]
pub enum LockInSettingsMessage {
    RefFreqChanged(ParameterInputMessage),
    TauChanged(ParameterInputMessage),
    LpfOrderChanged(ParameterInputMessage),
}

impl LockInSettingsWidget {
    pub fn new(default_settings: LockInSettings) -> Self {
        Self {
            settings: default_settings.clone(),
            ref_freq_input: ParameterInput::new(
                String::from("Reference frequency [Hz]"),
                default_settings.ref_freq,
            ),
            lpf_tau_input: ParameterInput::new(
                String::from("LPF Time constant [s]"),
                default_settings.lpf_tau,
            ),
            lpf_iir_order_input: ParameterInput::new(
                String::from("LPF Order [1-6]"),
                default_settings.lpf_iir_order as f32,
            ),
        }
    }

    pub fn update(&mut self, message: LockInSettingsMessage) -> Option<LockInSettings> {
        match message {
            LockInSettingsMessage::RefFreqChanged(new_value) => {
                if let Some(val) = self.ref_freq_input.update(new_value) {
                    self.settings.ref_freq = val;
                    return Some(self.settings.clone());
                }
                None
            }
            LockInSettingsMessage::TauChanged(new_value) => {
                if let Some(val) = self.lpf_tau_input.update(new_value) {
                    self.settings.lpf_tau = val;
                    return Some(self.settings.clone());
                }
                None
            }
            LockInSettingsMessage::LpfOrderChanged(new_value) => {
                if let Some(val) = self.lpf_iir_order_input.update(new_value) {
                    self.settings.lpf_iir_order = val as i32;
                    return Some(self.settings.clone());
                }
                None
            }
        }
    }

    pub fn view(&self) -> Element<LockInSettingsMessage> {
        let ref_freq_input = self
            .ref_freq_input
            .view()
            .map(LockInSettingsMessage::RefFreqChanged);
        let lpf_tau_input = self
            .lpf_tau_input
            .view()
            .map(LockInSettingsMessage::TauChanged);
        let lpf_iir_order_input = self
            .lpf_iir_order_input
            .view()
            .map(LockInSettingsMessage::LpfOrderChanged);

        column![
            text("LOCK-IN SETTINGS").size(20),
            ref_freq_input,
            lpf_tau_input,
            lpf_iir_order_input
        ]
        .into()
    }
}
