use std::mem;

use crate::emulations::c_elegans::rom::ROM;

/// Struct for representing a neuron connection
#[derive(Copy, Clone)]
struct NeuronConnection {
    id: u16,
    weight: i8,
}

/// Parse ROM word exactly like C
fn parse_rom_word(rom_word: u16) -> NeuronConnection {
    let [low, high] = rom_word.to_le_bytes();

    // uint16_t id = rom_byte[1] + ((rom_byte[0] & 0x80) << 1);
    let id = high as u16 + (((low & 0x80) as u16) << 1);

    // uint8_t weight_bits = rom_byte[0] & 0x7F;
    let mut weight_bits = low & 0x7F;

    // weight_bits = weight_bits + ((weight_bits & 0x40) << 1);
    weight_bits = weight_bits.wrapping_add((weight_bits & 0x40) << 1);

    let weight = weight_bits as i8;

    NeuronConnection { id, weight }
}

/// Connectome struct (layout mirrors C)
pub struct Connectome {
    neurons_tot: u16,
    muscles_tot: u8,

    neuron_current: Vec<i8>,
    neuron_next: Vec<i8>,

    muscle_current: Vec<i16>,
    muscle_next: Vec<i16>,

    // meta: [discharged_bit | idle_ticks(7 bits)]
    meta: Vec<u8>,
}

impl Connectome {
    /// Initialize connectome (ctm_init)
    pub fn new() -> Self {
        const CELLS: usize = 397; // example
        let neurons_tot = ROM[0] as u16;
        let muscles_tot = (CELLS as u16 - neurons_tot) as u8;

        let neurons_usize = neurons_tot as usize;
        let muscles_usize = muscles_tot as usize;

        Self {
            neurons_tot,
            muscles_tot,

            neuron_current: vec![0; neurons_usize],
            neuron_next: vec![0; neurons_usize],

            muscle_current: vec![0; muscles_usize],
            muscle_next: vec![0; muscles_usize],

            meta: vec![0; neurons_usize],
        }
    }

    /// Get current state (ctm_get_current_state)
    fn get_current_state(&self, id: u16) -> i16 {
        if id < self.neurons_tot {
            self.neuron_current[id as usize] as i16
        } else {
            self.muscle_current[(id - self.neurons_tot) as usize]
        }
    }

    /// Get next state (ctm_get_next_state)
    fn get_next_state(&self, id: u16) -> i16 {
        if id < self.neurons_tot {
            self.neuron_next[id as usize] as i16
        } else {
            self.muscle_next[(id - self.neurons_tot) as usize]
        }
    }

    /// Set next state with saturation (ctm_set_next_state)
    fn set_next_state(&mut self, id: u16, val: i16) {
        if id < self.neurons_tot {
            let v = if val > 127 {
                127
            } else if val < -128 {
                -128
            } else {
                val
            };
            self.neuron_next[id as usize] = v as i8;
        } else {
            self.muscle_next[(id - self.neurons_tot) as usize] = val;
        }
    }

    /// Add to next state (ctm_add_to_next_state)
    fn add_to_next_state(&mut self, id: u16, val: i8) {
        let curr = self.get_next_state(id);
        self.set_next_state(id, curr + val as i16);
    }

    /// Iterate state (ctm_iterate_state)
    fn iterate_state(&mut self) {
        self.neuron_current.copy_from_slice(&self.neuron_next);
        self.muscle_current.copy_from_slice(&self.muscle_next);
        self.muscle_next.fill(0);
    }

    /// Meta flag discharge (ctm_meta_flag_discharge)
    fn meta_flag_discharge(&mut self, id: u16, val: u8) {
        let idx = id as usize;
        if val == 0 {
            self.meta[idx] &= 0x7F;
        } else {
            self.meta[idx] = 0x80;
        }
    }

    /// Handle idle neurons (ctm_meta_handle_idle_neurons)
    fn meta_handle_idle_neurons(&mut self) {
        const MAX_IDLE: u8 = 100; // example
        for i in 0..self.neurons_tot {
            let idx = i as usize;

            let low = self.meta[idx] & 0x7F;
            let high = self.meta[idx] & 0x80;

            let mut idle_ticks = low;

            if self.get_next_state(i) == self.get_current_state(i) {
                self.meta[idx] = self.meta[idx].wrapping_add(1);
                idle_ticks = idle_ticks.wrapping_add(1);
            } else {
                self.meta[idx] = high;
            }

            if idle_ticks > MAX_IDLE {
                self.set_next_state(i, 0);
                self.meta[idx] = high;
            }
        }
    }

    /// Propagate connections (ctm_ping_neuron)
    fn ping_neuron(&mut self, id: u16) {
        let address = ROM[id as usize + 1];
        let end = ROM[id as usize + 2];
        let len = end - address;

        for i in 0..len {
            let rom_word = ROM[(address + i) as usize];
            let conn = parse_rom_word(rom_word);
            self.add_to_next_state(conn.id, conn.weight);
        }
    }

    /// Discharge neuron (ctm_discharge_neuron)
    fn discharge_neuron(&mut self, id: u16) {
        self.ping_neuron(id);
        self.set_next_state(id, 0);
    }

    /// Complete one neural cycle (ctm_neural_cycle)
    pub fn neural_cycle(&mut self, stim_neuron: Option<&[u16]>) {
        const THRESHOLD: i8 = 30; // example

        if let Some(stim) = stim_neuron {
            for &id in stim {
                self.ping_neuron(id);
            }
        }

        for i in 0..self.neurons_tot {
            if self.get_current_state(i) > THRESHOLD as i16 {
                self.discharge_neuron(i);
                self.meta_flag_discharge(i, 1);
            } else {
                self.meta_flag_discharge(i, 0);
            }
        }

        self.meta_handle_idle_neurons();
        self.iterate_state();
    }

    /// Get weight (ctm_get_weight)
    pub fn get_weight(&self, id: u16) -> i16 {
        self.get_current_state(id)
    }

    /// Get discharge flag (ctm_get_discharge)
    pub fn get_discharge(&self, id: u16) -> u8 {
        self.meta[id as usize] >> 7
    }
    pub fn discharge_query(&self, input_id: &[u16], query_result: &mut [u8]) {
        for i in 0..input_id.len() {
            let id = input_id[i] as usize;
            let discharged = self.meta[id] >> 7;
            query_result[i] = discharged;
        }
    }
}
