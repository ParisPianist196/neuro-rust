use crate::first_order_lif::FirstOrderLif;

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

pub fn min_max_step_range(min: f64, max: f64, step: f64) -> Vec<f64> {
    let int_range = min.abs() + max.abs();
    let num_steps = (int_range / step) as usize + 1; // +1 to include endpoint
    let inputs: Vec<f64> = (0..num_steps).map(|i| -1.0 + i as f64 * step).collect();
    inputs
}

// TODO 12/5/25 - Find a better place for this
pub fn compute_tuning_curve(
    neuron: &mut FirstOrderLif,
    inputs: &Vec<f64>,
    time_limit: f64,
    t_step: f64,
) -> Result<Vec<f64>, String> {
    let mut tuning_curve: Vec<f64> = vec![];
    let mut output;
    let mut count;

    let num_steps: i32 = (time_limit / t_step) as i32;
    for (_, input) in inputs.iter().enumerate() {
        // Simulate neuron for each input
        count = 0.;
        for _ in 0..num_steps {
            output = neuron.step(*input, t_step, None);
            if output > 0. {
                count += 1.;
            }
        }
        tuning_curve.push(count / time_limit)
    }
    Ok(tuning_curve)
}
