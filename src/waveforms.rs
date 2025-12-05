use plotly::{
    Plot, Scatter,
    common::{DashType::Dot, Line, Mode},
};
use wavegen::{Waveform, dc_bias, sine, wf};

pub trait PlotWaveform {
    fn new(sample_rate: f32, freq: f32, amp: f32, dc_bias: f32) -> Self;
    fn get_samples(&self, num_samples: usize) -> Vec<f64>;
    fn add_to_plot(&self, times: &Vec<f64>, plt: &mut Plot, name: &str);
}

pub struct Sine {
    waveform: Waveform<f64>,
}

impl PlotWaveform for Sine {
    fn new(sample_rate: f32, freq: f32, amp: f32, dc_bias: f32) -> Self {
        Sine {
            waveform: wf!(
                f64,
                sample_rate,
                sine!(frequency: freq, amplitude: amp),
                dc_bias!(dc_bias)
            ),
        }
    }
    fn get_samples(&self, num_samples: usize) -> Vec<f64> {
        self.waveform.iter().take(num_samples).collect()
    }
    fn add_to_plot(&self, times: &Vec<f64>, plt: &mut Plot, name: &str) {
        let trace = Scatter::new(times.clone(), self.get_samples(times.len()))
            .mode(Mode::Lines) // show as a line
            .line(Line::new().color("gray").width(2.0).dash(Dot))
            .name(name);
        plt.add_trace(trace);
    }
}
