use std::fs::File;

use crate::{
    connectome::Connectome,
    emulations::c_elegans::{neuron_ids::NeuronId, rom::ROM},
};
use std::io::Write;

pub fn test() -> Result<(), String> {
    const THRESHOLD: f32 = 30.;

    pub const MOTOR_NEURON_B: [u16; 18] = [
        NeuronId::DB1 as u16,
        NeuronId::DB2 as u16,
        NeuronId::DB3 as u16,
        NeuronId::DB4 as u16,
        NeuronId::DB5 as u16,
        NeuronId::DB6 as u16,
        NeuronId::DB7 as u16,
        NeuronId::VB1 as u16,
        NeuronId::VB2 as u16,
        NeuronId::VB3 as u16,
        NeuronId::VB4 as u16,
        NeuronId::VB5 as u16,
        NeuronId::VB6 as u16,
        NeuronId::VB7 as u16,
        NeuronId::VB8 as u16,
        NeuronId::VB9 as u16,
        NeuronId::VB10 as u16,
        NeuronId::VB11 as u16,
    ];

    pub const MOTOR_NEURON_A: [u16; 21] = [
        NeuronId::DA1 as u16,
        NeuronId::DA2 as u16,
        NeuronId::DA3 as u16,
        NeuronId::DA4 as u16,
        NeuronId::DA5 as u16,
        NeuronId::DA6 as u16,
        NeuronId::DA7 as u16,
        NeuronId::DA8 as u16,
        NeuronId::DA9 as u16,
        NeuronId::VA1 as u16,
        NeuronId::VA2 as u16,
        NeuronId::VA3 as u16,
        NeuronId::VA4 as u16,
        NeuronId::VA5 as u16,
        NeuronId::VA6 as u16,
        NeuronId::VA7 as u16,
        NeuronId::VA8 as u16,
        NeuronId::VA9 as u16,
        NeuronId::VA10 as u16,
        NeuronId::VA11 as u16,
        NeuronId::VA12 as u16,
    ];

    let nose_touch_neurons: Vec<u16> = vec![
        NeuronId::FLPR as u16,
        NeuronId::FLPL as u16,
        NeuronId::ASHL as u16,
        NeuronId::ASHR as u16,
        NeuronId::IL1VL as u16,
        NeuronId::IL1VR as u16,
        NeuronId::OLQDL as u16,
        NeuronId::OLQDR as u16,
        NeuronId::OLQVR as u16,
        NeuronId::OLQVL as u16,
    ];

    let chemotaxis_neurons = [
        NeuronId::ADFL as u16,
        NeuronId::ADFR as u16,
        NeuronId::ASGR as u16,
        NeuronId::ASGL as u16,
        NeuronId::ASIL as u16,
        NeuronId::ASIR as u16,
        NeuronId::ASJR as u16,
        NeuronId::ASJL as u16,
    ];

    let mut out_file = File::create("./motor_ab.dat").map_err(|err| err.to_string())?;
    let f = File::open("./src/emulations/c_elegans/CElegansNeuronTables/Connectome.csv")
        .map_err(|err| err.to_string())?;
    let mut connectome = Connectome::new();
    let mut motor_a_result: Vec<u8> = vec![0; MOTOR_NEURON_A.len()];
    let mut motor_b_result: Vec<u8> = vec![0; MOTOR_NEURON_B.len()];

    // Perform burn in
    for _ in 0..1000 {
        connectome.neural_cycle(Some(&chemotaxis_neurons));
    }

    // Run  100 cycles of chemotaxis
    for _ in 0..1000 {
        connectome.neural_cycle(Some(&chemotaxis_neurons));
        connectome.discharge_query(&MOTOR_NEURON_B, &mut motor_b_result);
        connectome.discharge_query(&MOTOR_NEURON_A, &mut motor_a_result);
        print_motor_ab_discharges(&mut out_file, &motor_a_result, &motor_b_result)
            .map_err(|err| err.to_string())?;
    }

    for _ in 0..1000 {
        connectome.neural_cycle(Some(&nose_touch_neurons));
        connectome.discharge_query(&MOTOR_NEURON_B, &mut motor_b_result);
        connectome.discharge_query(&MOTOR_NEURON_A, &mut motor_a_result);
        print_motor_ab_discharges(&mut out_file, &motor_a_result, &motor_b_result)
            .map_err(|err| err.to_string())?;
    }
    Ok(())
}

fn print_motor_ab_discharges<W: Write>(
    mut w: W,
    a: &Vec<u8>,
    b: &Vec<u8>,
) -> Result<(), std::io::Error> {
    for i in 0..a.len() {
        write!(w, "{} ", a[i])?;
    }

    for i in 0..b.len() - 1 {
        write!(w, "{} ", b[i])?;
    }

    writeln!(w, "{}", b[b.len() - 1])?;
    Ok(())
}
