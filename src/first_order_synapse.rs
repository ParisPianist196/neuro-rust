use plotly::{
    Plot, Scatter,
    common::{Line, Mode},
};

use crate::first_order_li::FirstOrderLi;

pub struct FirstOrderSynapse {
    pub tau_s: f64,
    pub output: f64,

    pub output_history: Vec<f64>,
}

impl Default for FirstOrderSynapse {
    fn default() -> Self {
        FirstOrderSynapse {
            tau_s: 0.0,
            output: 0.0,
            output_history: vec![],
        }
    }
}

impl FirstOrderSynapse {
    pub fn step(&mut self, i: f64, t_step: f64) -> f64 {
        self.output = self.output * (1.0 - t_step / self.tau_s) + i * t_step / self.tau_s;
        self.output_history.push(self.output);
        self.output
    }

    pub fn reset(&mut self) {
        self.output = 0.;
    }

    pub fn add_output_history_to_plot(&self, times: &Vec<f64>, plt: &mut Plot) {
        let trace = Scatter::new(times.clone(), self.output_history.clone())
            .mode(Mode::Lines)
            .line(Line::new().color("blue").width(2.0))
            .name("Output");
        plt.add_trace(trace);
    }

    pub fn add_decoded_output_to_plot(&self, times: &Vec<f64>, phi: f64, plt: &mut Plot) {
        let decoded: Vec<f64> = self.output_history.iter().map(|x| phi * x).collect();
        let trace = Scatter::new(times.clone(), decoded)
            .mode(Mode::Lines)
            .line(Line::new().color("blue").width(2.0))
            .name("Decoded Output");
        plt.add_trace(trace);
    }
}
