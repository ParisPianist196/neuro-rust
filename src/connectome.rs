use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::num::ParseFloatError;

#[derive(Debug, Clone)]
pub struct Connection {
    pub target: usize,
    pub weight: f32,
    pub synapse_type: String,
    pub neurotransmitter: String,
}

#[derive(Debug)]
pub struct Connectome {
    // Double buffering
    pub neuron_current_buffer: Vec<f32>,
    pub neuron_next_buffer: Vec<f32>,

    pub neuron_map: HashMap<String, usize>, // name → ID
    pub adjacency: Vec<Vec<Connection>>,    // outgoing edges

    pub threshold: f32,
}

impl Connectome {
    pub fn new(mut csvfile: File, threshold: f32) -> Result<Self, String> {
        // Read file into string
        let mut contents = String::new();
        csvfile
            .read_to_string(&mut contents)
            .map_err(|err| err.to_string())?;

        let mut reader = csv::Reader::from_reader(contents.as_bytes());

        let mut neuron_map: HashMap<String, usize> = HashMap::new();
        let mut adjacency: Vec<Vec<Connection>> = Vec::new();

        // Function: get or create neuron ID
        let get_id = |name: &str,
                      neuron_map: &mut HashMap<String, usize>,
                      adjacency: &mut Vec<Vec<Connection>>| {
            if let Some(&id) = neuron_map.get(name) {
                id
            } else {
                let id = neuron_map.len();
                neuron_map.insert(name.to_string(), id);
                adjacency.push(Vec::new());
                id
            }
        };

        for result in reader.records() {
            let record = result.map_err(|err| err.to_string())?;

            let origin_name = &record[0];
            let target_name = &record[1];
            let syn_type = record[2].to_string();
            let weight: f32 = record[3]
                .parse()
                .map_err(|err: ParseFloatError| err.to_string())?;
            let neurotransmitter = record[4].to_string();

            // Assign IDs
            let origin_id = get_id(origin_name, &mut neuron_map, &mut adjacency);
            let target_id = get_id(target_name, &mut neuron_map, &mut adjacency);

            // Push connection
            adjacency[origin_id].push(Connection {
                target: target_id,
                weight,
                synapse_type: syn_type,
                neurotransmitter,
            });
        }

        Ok(Self {
            neuron_next_buffer: vec![],
            neuron_current_buffer: vec![],
            neuron_map,
            adjacency,
            threshold,
        })
    }

    /// Advance the neural system by one tick.
    /// `stimulated` is a list of neuron names to directly stimulate.
    pub fn step(&mut self, stimulated: &[&str]) {
        // 1. Apply external stimulation
        for &name in stimulated {
            if let Some(&id) = self.neuron_map.get(name) {
                self.ping_neuron(id);
            }
        }

        // 2. Check for discharges
        for id in 0..self.neuron_current_buffer.len() {
            if self.neuron_current_buffer[id] > self.threshold {
                self.discharge_neuron(id);
            }
        }

        // 3. Swap buffers
        std::mem::swap(
            &mut self.neuron_current_buffer,
            &mut self.neuron_next_buffer,
        );

        // 4. Clear next buffer for the next tick
        self.neuron_next_buffer.fill(0.);
    }

    /// Propagate all outgoing connections of a neuron
    fn ping_neuron(&mut self, id: usize) {
        for conn in &self.adjacency[id] {
            self.neuron_next_buffer[conn.target] += conn.weight;
        }
    }

    /// A neuron fires: propagate, then reset its value
    fn discharge_neuron(&mut self, id: usize) {
        self.ping_neuron(id);
        self.neuron_next_buffer[id] = 0.;
    }
}
