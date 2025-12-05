pub struct FirstOrderSynapse {
    pub tau_s: f64,
    pub output: f64,
}

impl FirstOrderSynapse {
    pub fn step(&mut self, i: f64, t_step: f64) -> f64 {
        self.output = self.output * (1.0 - t_step / self.tau_s) + i * t_step / self.tau_s;
        self.output
    }
}
