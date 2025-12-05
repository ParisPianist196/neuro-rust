pub fn square_wave_at_t(t: &f64, off_time_interval: i32, on_time_interval: i32) -> f64 {
    if t.rem_euclid((off_time_interval + on_time_interval) as f64) < off_time_interval as f64 {
        1.0
    } else {
        0.0
    }
}

pub fn scale_to_range(data: &Vec<f64>, min_val: f64, max_val: f64) -> Vec<f64> {
    let current_min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let current_max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    data.iter()
        .map(|&x| min_val + (x - current_min) * (max_val - min_val) / (current_max - current_min))
        .collect()
}
