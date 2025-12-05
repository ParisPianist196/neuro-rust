pub fn square_wave_at_t(t: &f64, off_time_interval: i32, on_time_interval: i32) -> f64 {
    if t.rem_euclid((off_time_interval + on_time_interval) as f64) < off_time_interval as f64 {
        1.0
    } else {
        0.0
    }
}
