use crate::lify_stuff::simulation::SynapseSimulation;

pub struct FirstOrderSynapse {
    pub tau_s: f64,
    pub output: f64,
}

impl Default for FirstOrderSynapse {
    fn default() -> Self {
        FirstOrderSynapse {
            tau_s: 0.0,
            output: 0.0,
        }
    }
}

impl FirstOrderSynapse {
    pub fn step(&mut self, i: f64, t_step: f64, simulation: Option<&mut SynapseSimulation>) -> f64 {
        self.output = self.output * (1.0 - t_step / self.tau_s) + i * t_step / self.tau_s;
        match simulation {
            Some(sim) => sim.output.record(self.output),
            None => (),
        }
        self.output
    }

    pub fn reset(&mut self) {
        self.output = 0.;
    }
}
