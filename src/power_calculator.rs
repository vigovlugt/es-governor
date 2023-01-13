use crate::consts::{BIG_FREQUENCIES, LITTLE_FREQUENCIES};

const BASE_POWER: f64 = 0.38;
const GPU_AMP: f64 = 0.26;

const LITTLE_A: f64 = 0.03004633;
const LITTLE_B: f64 = 1.60189609;
const LITTLE_C: f64 = 0.37310575;

const BIG_A: f64 = 0.08;
const BIG_B: f64 = 1.57;
const BIG_C: f64 = 0.032;

pub fn calculate_power_amp(use_gpu: bool, little_freq: Option<i32>, big_freq: Option<i32>) -> f64 {
    let little_power = match little_freq {
        Some(freq) => {
            LITTLE_C + LITTLE_A * LITTLE_B.powf((freq as f64) / (LITTLE_FREQUENCIES[0] as f64))
        }
        None => 0.0,
    };
    let big_power = match big_freq {
        Some(freq) => BIG_C + BIG_A * BIG_B.powf((freq as f64) / (BIG_FREQUENCIES[0] as f64)),
        None => 0.0,
    };
    let gpu_power = if use_gpu { GPU_AMP } else { 0.0 };

    return BASE_POWER + big_power + gpu_power + little_power;
}
