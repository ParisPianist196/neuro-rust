mod connectome;
pub mod emulations;
mod first_order_lif;
mod first_order_lif_collection;
mod first_order_synapse;
mod first_order_synapses_collection;
mod history;
mod simulation;
mod utils;
mod waveforms;
use crate::emulations::c_elegans::c_elegans_nematode::test;

pub fn main() {
    let _ = test();
}
