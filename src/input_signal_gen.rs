use rand::prelude::*;
use std::f32::consts::PI;

use crate::input_signal_settings::InputSignalSettings;

pub fn generate_input_signal(settings: InputSignalSettings) -> Vec<f32> {
    let len = ((settings.len_sec as f32) * settings.fs)
        .round()
        .max(0.0) as usize;

    let signal = make_sin(settings.sig_ampl, settings.sig_freq, settings.sig_phase_deg, settings.fs, len);

    let mut rng = rand::rng();
    let noise: Vec<f32> = (0..len).map(|_| rng.random_range(-1.0..1.0) * settings.noise_ampl).collect();

    return signal.iter().zip(noise.iter()).map(|(&s, &n)| s + n).collect();
}

pub fn generate_input_time_axis(settings: InputSignalSettings) -> Vec<f32> {
    let len = ((settings.len_sec as f32) * settings.fs)
        .round()
        .max(0.0) as usize;
    (0..len).map(|i| (i as f32) * (1.0 / settings.fs)).collect()
}

pub fn make_sin(ampl: f32, freq: f32, phase_deg: f32, sampling_freq: f32, len: usize) -> Vec<f32> {
    let mut v : Vec<f32> = Vec::new();
    for i in 0..len {
        let t = i as f32 * (1.0 / sampling_freq);
        let x = (2.0 * PI * freq) * t + (phase_deg * PI / 180.0);
        v.push(ampl * x.sin());
    }
    v
}