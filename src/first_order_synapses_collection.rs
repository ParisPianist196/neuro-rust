use crate::first_order_synapse::FirstOrderSynapse;

pub struct FirstOrderSynapsesCollection {
    pub synapses: Vec<FirstOrderSynapse>,
}

impl FirstOrderSynapsesCollection {
    pub fn new(num_synapses: i32, tau_s: f64) -> Self {
        let synapses = (0..num_synapses)
            .map(|i| {
                let mut s = FirstOrderSynapse::default();
                s.tau_s = tau_s;
                s
            })
            .collect();
        FirstOrderSynapsesCollection { synapses }
    }

    pub fn step(&mut self, inputs: &Vec<f64>, t_step: f64) -> Result<Vec<f64>, String> {
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
            .map(|(i, synapse)| synapse.step(inputs[i], t_step))
            .collect())
    }

    fn reset(&mut self) {
        for syn in self.synapses.iter_mut() {
            syn.reset();
        }
    }
}
