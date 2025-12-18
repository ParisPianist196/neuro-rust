use nalgebra::{DMatrix, DVector};
use rand::seq::IndexedRandom;
use rand::{Rng, SeedableRng};

use crate::lify_stuff::first_order_lif::FirstOrderLif;
use crate::lify_stuff::simulation::NeuronSimulationCollection;

pub struct FirstOrderLifCollection {
    pub neurons: Vec<FirstOrderLif>,
}

impl FirstOrderLifCollection {
    pub fn new(
        num_neurons: usize,
        tau_rc: f64,
        tau_ref: f64,
        max_rate_range: (f64, f64),
        intercept_range: (f64, f64),
        encoder_options: Vec<i32>,
    ) -> Result<Self, String> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0);
        let mut neurons = vec![];
        for _ in 0..num_neurons {
            let max_rate: f64 = rng.random_range(max_rate_range.0..max_rate_range.1);
            let intercept: f64 = rng.random_range(intercept_range.0..intercept_range.1);
            let encoder = encoder_options
                .choose(&mut rng)
                .ok_or("Unable to choose encoder for neuron.".to_string())?;
            let mut lif = FirstOrderLif::new(max_rate, intercept, *encoder);
            lif.tau_rc = tau_rc;
            lif.tau_ref = tau_ref;
            neurons.push(lif);
        }
        Ok(Self { neurons })
    }

    pub fn step(
        &mut self,
        input: f64,
        t_step: f64,
        mut sim_collection: Option<&mut NeuronSimulationCollection>,
    ) -> Vec<f64> {
        let mut outputs: Vec<f64> = vec![];
        // todo 12/6/25: Parallelize
        for (i, n) in self.neurons.iter_mut().enumerate() {
            let sim_for_this_neuron = sim_collection.as_mut().map(|coll| coll.get(i));
            let output = n.step(
                input * n.gain * n.encoder as f64 + n.bias,
                t_step,
                sim_for_this_neuron,
            );
            outputs.push(output);
        }
        outputs
    }

    pub fn reset(&mut self) {
        for n in self.neurons.iter_mut() {
            n.reset();
        }
    }

    pub fn get_tuning_curves(&self, inputs: &Vec<f64>) -> Vec<Vec<f64>> {
        self.neurons
            .iter()
            .map(|n| {
                let mut curve: Vec<f64> = vec![];
                for input in inputs.clone() {
                    curve.push(n.analytical_rate(input * n.gain * n.encoder as f64 + n.bias));
                }
                curve
            })
            .collect()
    }

    pub fn get_decoders(&self, inputs: &Vec<f64>) -> Result<Vec<f64>, String> {
        let tuning_curves = self.get_tuning_curves(inputs);
        let n_neurons = tuning_curves.len();
        let n_inputs = inputs.len();

        // A: (neurons × inputs) matrix
        let mut neuron_cross_inputs_mat = DMatrix::<f64>::zeros(n_neurons, n_inputs);
        for (i, curve) in tuning_curves.iter().enumerate() {
            for (j, val) in curve.iter().enumerate() {
                neuron_cross_inputs_mat[(i, j)] = *val;
            }
        }

        let value = DVector::<f64>::from_vec(inputs.clone());
        let gamma = &neuron_cross_inputs_mat * neuron_cross_inputs_mat.transpose()
            + DMatrix::<f64>::identity(n_neurons, n_neurons);
        let gamma_inv = gamma.try_inverse().ok_or("Unable to get the inverseof the matrix when computing decoders for first order lif collection.")?;
        let upsilon = &neuron_cross_inputs_mat * value;
        let phi: DVector<f64> = gamma_inv * upsilon;

        Ok(phi.iter().cloned().collect::<Vec<f64>>())
    }
}
