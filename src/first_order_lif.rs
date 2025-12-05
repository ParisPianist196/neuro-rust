use plotly::{
    Layout, Plot, Scatter,
    common::{DashType::Dot, Line, Mode},
    layout::{Shape, ShapeLine},
};

use crate::utils::scale_to_range;

pub struct FirstOrderLif {
    pub tau_rc: f64,
    pub v: f64,
    pub v_th: f64,
    pub tau_ref: f64,
    pub output: f64,
    pub refractory_time: f64,
    pub gain: f64,
    pub bias: f64,
    pub encoder: i32,

    pub v_history: Vec<f64>,
    pub v_th_history: Vec<f64>,
    pub output_history: Vec<f64>,
}

impl FirstOrderLif {
    pub fn new(max_rate: f64, intercept: f64, encoder: i32) -> Self {
        let mut lif = Self {
            tau_rc: 0.2,
            v: 0.,
            v_th: 1.,
            tau_ref: 0.002,
            output: 0.,
            refractory_time: 0.,
            bias: 0.,
            gain: 0.,
            encoder: encoder,

            v_history: vec![],
            v_th_history: vec![],
            output_history: vec![],
        };

        let (gain, bias) = lif.gain_bias(max_rate, intercept);
        lif.gain = gain;
        lif.bias = bias;
        lif
    }

    pub fn step(&mut self, i: f64, t_step: f64) -> f64 {
        self.refractory_time -= t_step;

        if self.refractory_time < 0. {
            self.v = self.v * (1. - t_step / self.tau_rc) + i * t_step / self.tau_rc;
        }

        if self.v > self.v_th {
            self.refractory_time = self.tau_ref;
            self.output = 1.0 / t_step;
            self.v = 0.;
        } else {
            self.output = 0.
        }

        // Update history for plotting
        self.v_history.push(self.v);
        self.v_th_history.push(self.v_th);
        self.output_history.push(self.output);

        self.output
    }

    pub fn analytical_rate(&self, input: f64) -> f64 {
        if input <= self.v_th {
            0.
        } else {
            1. / (self.tau_ref - self.tau_rc * (1. - self.v_th / input).ln())
        }
    }

    pub fn decoder(&self, range_low: f64, range_high: f64, interval: f64) -> f64 {
        let mut numerator = 0.;
        let mut denominator = 0.;
        let mut i = range_low;
        let mut r: f64;
        while i < range_high {
            r = self.analytical_rate(i);
            numerator += r * i;
            denominator += r * r;
            i += interval;
        }
        numerator / denominator
    }

    pub fn reset(&mut self) {
        self.v = 0.;
        self.output = 0.;
        self.refractory_time = 0.;
    }

    pub fn gain_bias(&self, max_rate: f64, intercept: f64) -> (f64, f64) {
        let gain = self.v_th
            * (1. - 1. / (1. - ((self.tau_ref - 1. / max_rate) / self.tau_ref).exp()))
            / (intercept - 1.);
        let bias = self.v_th - gain * intercept;
        (gain, bias)
    }

    pub fn add_v_history_to_plot(&self, times: &Vec<f64>, plt: &mut Plot) {
        let trace = Scatter::new(times.clone(), self.v_history.clone())
            .mode(Mode::Lines)
            .line(Line::new().color("blue").width(2.0))
            .name("V");
        plt.add_trace(trace);
    }

    pub fn add_v_th_history_to_plot(&self, times: &Vec<f64>, plt: &mut Plot) {
        let trace = Scatter::new(times.clone(), self.v_th_history.clone())
            .mode(Mode::Lines)
            .line(Line::new().color("green").width(2.0).dash(Dot))
            .name("V_th");
        plt.add_trace(trace);
    }

    pub fn add_output_history_to_plot(&self, times: &Vec<f64>, plt: &mut Plot, scale: bool) {
        let mut temp_output = self.output_history.clone();
        if scale {
            temp_output = scale_to_range(&temp_output, 0.0, 1.0);
        }

        let trace = Scatter::new(times.clone(), temp_output)
            .mode(Mode::Lines)
            .line(Line::new().color("yellow").width(2.0))
            .name("Output");
        plt.add_trace(trace);
    }

    pub fn add_spikes_to_plot(&self, times: &Vec<f64>, plt: &mut Plot) -> Result<(), String> {
        if times.len() != self.output_history.len() {
            return Err(
                "Cannot add spikes to plot. Mismatch between times dimension and neuron output history".to_string(),
            );
        }
        let vlines: Vec<Shape> = self
            .output_history
            .iter()
            .enumerate()
            .filter_map(|(i, &output)| {
                if output > 0.0 {
                    Some(
                        Shape::new()
                            .x0(times[i])
                            .x1(times[i])
                            .y0(0.0) // adjust y-range as needed
                            .y1(1.0)
                            .line(ShapeLine::new().color("red").width(1.0))
                            .name("Spikes"),
                    )
                } else {
                    None
                }
            })
            .collect();
        let label_trace = Scatter::new(vec![0.0], vec![0.0])
            .mode(Mode::Lines)
            .line(Line::new().color("red").width(1.0))
            .name("Spikes");

        plt.add_trace(label_trace);

        let layout = Layout::new().shapes(vlines);
        plt.set_layout(layout);
        Ok(())
    }
}
