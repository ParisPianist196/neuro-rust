use crate::first_order_lif::FirstOrderLif;
use rand::Rng;
use rand::seq::IndexedRandom;

pub struct FirstOrderLifCollection {
    neurons: Vec<FirstOrderLif>,
}

impl FirstOrderLifCollection {
    pub fn new(
        num_neurons: i32,
        max_rate_range: (f64, f64),
        intercept_range: (f64, f64),
        encoder_options: Vec<i32>,
    ) -> Result<Self, String> {
        let mut rng = rand::rng();
        let mut neurons = vec![];
        for _ in 0..num_neurons {
            let max_rate: f64 = rng.random_range(max_rate_range.0..max_rate_range.1);
            let intercept: f64 = rng.random_range(intercept_range.0..intercept_range.1);
            let encoder = encoder_options
                .choose(&mut rng)
                .ok_or("Unable to choose encoder for neuron.".to_string())?;
            neurons.push(FirstOrderLif::new(max_rate, intercept, *encoder));
        }
        Ok(Self { neurons })
    }

    pub fn step(&mut self, input: f64, t_step: f64) {
        let mut outputs: Vec<f64> = vec![];
        for n in self.neurons.iter_mut() {
            let output = n.step(input * n.gain * n.encoder as f64 + n.bias, t_step);
            outputs.push(output);
        }
    }

    pub fn reset(&mut self) {
        for n in self.neurons.iter_mut() {
            n.reset();
        }
    }
}
