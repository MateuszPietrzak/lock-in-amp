use crate::input_signal_gen::make_sin;
use crate::lock_in_settings::LockInSettings;


pub fn analyze(input_signal: &Vec<f32>, params: &LockInSettings) -> (Vec<f32>, Vec<f32>) {
    assert!(!input_signal.is_empty(), "No signal to analyze.");

    let ref_i = make_sin(1.0, params.ref_freq, 0.0, params.fs, input_signal.len());
    let ref_q = make_sin(1.0, params.ref_freq, 90.0, params.fs, input_signal.len());

    let mut mul_i: Vec<f32> =  input_signal.iter().zip(ref_i.iter()).map(|(&s, &r)| s * r).collect();
    let mut mul_q: Vec<f32> =  input_signal.iter().zip(ref_q.iter()).map(|(&s, &r)| s * r).collect();

    for _ in 0..params.lpf_iir_order {
        apply_iir_lpf(&mut mul_i, params.lpf_tau, params.fs);
        apply_iir_lpf(&mut mul_q, params.lpf_tau, params.fs);
    }
    
    let measured_ampl: Vec<f32> = mul_i.iter().zip(mul_q.iter()).map(|(&i, &q)| 2.0 * (i * i + q * q).sqrt()).collect();
    let measured_phase: Vec<f32> = mul_i.iter().zip(mul_q.iter()).map(|(&i, &q)| (q).atan2(i)).collect();
    
    (measured_ampl, measured_phase)
}

fn apply_iir_lpf(y: &mut Vec<f32> , tau_s: f32, fs_hz: f32) {
    assert!(!y.is_empty(), "No signal to filter.");
    let alpha : f32 = 1.0 - (-1.0/(tau_s * fs_hz)).exp();
    let mut y_prev : f32 = y[0];
    for i in 0..y.len() {
        y[i] = alpha * y[i] + (1.0 - alpha) * y_prev;
        y_prev = y[i];
    }
}

fn get_iir_settling_time_sec(order: i32, tau: f32) -> f32 {
    let multiplier = match order {
        1 => 4.6,
        2 => 6.6,
        3 => 8.4,
        4 => 10.0,
        5 => 11.6,
        6 => 13.0,
        _ => {
            panic!("Bro you dont need more than 6th order lol");
        }
    };
    multiplier * tau
}