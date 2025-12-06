use crate::{first_order_synapse::FirstOrderSynapse, simulation::SynapseSimulationCollection};

pub struct FirstOrderSynapsesCollection {
    pub synapses: Vec<FirstOrderSynapse>,
}

impl FirstOrderSynapsesCollection {
    pub fn new(num_synapses: usize, tau_s: f64) -> Self {
        let synapses = (0..num_synapses)
            .map(|_| {
                let mut s = FirstOrderSynapse::default();
                s.tau_s = tau_s;
                s
            })
            .collect();
        FirstOrderSynapsesCollection { synapses }
    }

    pub fn step(
        &mut self,
        inputs: &Vec<f64>,
        t_step: f64,
        mut sim_collection: Option<&mut SynapseSimulationCollection>,
    ) -> Result<Vec<f64>, String> {
        if inputs.len() != self.synapses.len() {
            return Err(format!(
                "Incorrect number of inputs. Synapse expects, {}",
                self.synapses.len()
            ));
        }
        Ok(self
            .synapses
            .iter_mut()
            .enumerate()
            .map(|(i, synapse)| {
                let sim_for_this_neuron = sim_collection.as_mut().map(|coll| coll.get(i));
                synapse.step(inputs[i], t_step, sim_for_this_neuron)
            })
            .collect())
    }

    fn reset(&mut self) {
        for syn in self.synapses.iter_mut() {
            syn.reset();
        }
    }
}
