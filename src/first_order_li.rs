use plotly::{
    Layout, Plot, Scatter,
    common::{DashType::Dot, Line, Mode},
    layout::{Shape, ShapeLine},
};

use crate::utils::scale_to_range;

pub struct FirstOrderLi {
    pub tau_rc: f64,
    pub v: f64,
    pub v_th: f64,
    pub tau_ref: f64,
    pub output: f64,
    pub refractory_time: f64,

    pub v_history: Vec<f64>,
    pub v_th_history: Vec<f64>,
    pub output_history: Vec<f64>,
}

impl Default for FirstOrderLi {
    fn default() -> Self {
        Self {
            tau_rc: 0.2,
            v: 0.0,
            v_th: 1.0,
            tau_ref: 0.002,
            output: 0.0,
            refractory_time: 0.0,

            v_history: vec![],
            v_th_history: vec![],
            output_history: vec![],
        }
    }
}

impl FirstOrderLi {
    pub fn step(&mut self, i: f64, t_step: f64) {
        self.refractory_time -= t_step;

        if self.refractory_time < 0.0 {
            self.v = self.v * (1.0 - t_step / self.tau_rc) + i * t_step / self.tau_rc;
        }

        if self.v > self.v_th {
            self.refractory_time = self.tau_ref;
            self.output = 1.0 / t_step;
            self.v = 0.0;
        } else {
            self.output = 0.0
        }

        // Update history for plotting
        self.v_history.push(self.v);
        self.v_th_history.push(self.v_th);
        self.output_history.push(self.output);
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
