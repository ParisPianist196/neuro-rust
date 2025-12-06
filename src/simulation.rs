use nalgebra::DMatrix;

use crate::history::History;
#[derive(Default, Debug)]
pub struct NeuronSimulation {
    pub v: History,
    pub output: History,
    pub v_th: History,
}

#[derive(Default, Debug)]
pub struct SynapseSimulation {
    pub output: History,
}

pub struct SynapseSimulationCollection {
    sims: Vec<SynapseSimulation>,
}

impl SynapseSimulationCollection {
    pub fn new(num_neurons: usize) -> Self {
        Self {
            sims: (0..num_neurons)
                .map(|_| SynapseSimulation::default())
                .collect(),
        }
    }

    pub fn get(&mut self, i: usize) -> &mut SynapseSimulation {
        &mut self.sims[i]
    }

    pub fn all(&self) -> &[SynapseSimulation] {
        &self.sims
    }
    pub fn output_matrix(&self) -> Result<DMatrix<f64>, String> {
        if self.sims.is_empty() {
            return Err("No synapses".into());
        }

        let steps = self.sims[0].output.get().len();
        let syns = self.sims.len();

        // Check same length
        for (i, s) in self.sims.iter().enumerate() {
            if s.output.get().len() != steps {
                return Err(format!(
                    "Synapse {} has {}, expected {}",
                    i,
                    s.output.get().len(),
                    steps
                ));
            }
        }

        // Column-major order for nalgebra
        let mut data = Vec::with_capacity(steps * syns);
        for s in &self.sims {
            data.extend(s.output.get());
        }

        Ok(DMatrix::from_column_slice(steps, syns, &data))
    }
}

pub struct NeuronSimulationCollection {
    sims: Vec<NeuronSimulation>,
}

impl NeuronSimulationCollection {
    pub fn new(num_neurons: usize) -> Self {
        Self {
            sims: (0..num_neurons)
                .map(|_| NeuronSimulation::default())
                .collect(),
        }
    }

    pub fn get(&mut self, i: usize) -> &mut NeuronSimulation {
        &mut self.sims[i]
    }

    pub fn all(&self) -> &[NeuronSimulation] {
        &self.sims
    }
}
pub struct ExperimentSimulation {
    pub neurons: NeuronSimulationCollection,
    pub synapses: SynapseSimulationCollection,
}

impl ExperimentSimulation {
    pub fn new(num_neurons: usize, num_synapses: usize) -> Self {
        Self {
            neurons: NeuronSimulationCollection::new(num_neurons),
            synapses: SynapseSimulationCollection::new(num_synapses),
        }
    }
    pub fn synapse_outputs(&self) -> Result<DMatrix<f64>, String> {
        self.synapses.output_matrix()
    }
}

pub struct MultiSimulation {
    experiments: Vec<ExperimentSimulation>,
}

impl MultiSimulation {
    pub fn new(num_experiments: usize, num_neurons: usize, num_synapses: usize) -> Self {
        Self {
            experiments: (0..num_experiments)
                .map(|_| ExperimentSimulation::new(num_neurons, num_synapses))
                .collect(),
        }
    }

    pub fn get_experiment(&mut self, i: usize) -> &mut ExperimentSimulation {
        &mut self.experiments[i]
    }
}
