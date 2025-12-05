pub struct FirstOrderLi {
    pub tau_rc: f64,
    pub v: f64,
    pub v_th: f64,
    pub tau_ref: f64,
    pub output: f64,
    pub refractory_time: f64,
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
    }
}
