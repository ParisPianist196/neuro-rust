use nalgebra::clamp;

use crate::lify_stuff::simulation::NeuronSimulation;

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
        };

        let (gain, bias) = lif.gain_bias(max_rate, intercept);
        lif.gain = gain;
        lif.bias = bias;
        lif
    }

    pub fn step(&mut self, i: f64, t_step: f64, sim: Option<&mut NeuronSimulation>) -> f64 {
        self.refractory_time -= t_step;

        let delta_t = clamp(t_step - self.refractory_time, 0., t_step);

        self.v = i + (self.v - i) * (-delta_t / self.tau_rc).exp();

        if self.v > self.v_th {
            let spike_time = delta_t + self.tau_rc * ((self.v - i) / (self.v_th - i)).log(10.);
            self.refractory_time = self.tau_ref + spike_time;
            self.output = 1.0 / t_step;
            self.v = 0.;
        } else {
            self.output = 0.
        }

        // Update history for plotting
        if let Some(sim) = sim {
            sim.v.record(self.v);
            sim.v_th.record(self.v_th);
            sim.output.record(self.output);
        }

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
            * (1. - 1. / (1. - ((self.tau_ref - 1. / max_rate) / self.tau_rc).exp()))
            / (intercept - 1.);
        let bias = self.v_th - gain * intercept;
        (gain, bias)
    }
}
