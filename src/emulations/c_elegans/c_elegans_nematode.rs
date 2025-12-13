use std::fs::File;

use crate::connectome::Connectome;
use std::io::Write;

pub fn test() -> Result<(), String> {
    const THRESHOLD: f32 = 30.;

    pub const MOTOR_NEURON_B: &[&str] = &[
        "DB1", "DB2", "DB3", "DB4", "DB5", "DB6", "DB7", "VB1", "VB2", "VB3", "VB4", "VB5", "VB6",
        "VB7", "VB8", "VB9", "VB10", "VB11",
    ];

    pub const MOTOR_NEURON_A: &[&str] = &[
        "DA1", "DA2", "DA3", "DA4", "DA5", "DA6", "DA7", "DA8", "DA9", "VA1", "VA2", "VA3", "VA4",
        "VA5", "VA6", "VA7", "VA8", "VA9", "VA10", "VA11", "VA12",
    ];

    let nose_touch_neurons = vec![
        "FLPR", "FLPL", "ASHL", "ASHR", "IL1VL", "IL1VR", "OLQDL", "OLQDR", "OLQVR", "OLQV",
    ];

    let chemotaxis_neurons = [
        "ADFL", "ADFR", "ASGR", "ASGL", "ASIL", "ASIR", "ASJR", "ASJL",
    ];

    let mut out_file = File::create("./motor_ab.dat").map_err(|err| err.to_string())?;
    let f = File::open("./src/emulations/c_elegans/CElegansNeuronTables/Connectome.csv")
        .map_err(|err| err.to_string())?;
    let mut connectome = Connectome::new(f, THRESHOLD)?;
    let mut motor_a_result: Vec<bool> = vec![false; MOTOR_NEURON_A.len()];
    let mut motor_b_result: Vec<bool> = vec![false; MOTOR_NEURON_B.len()];

    // Perform burn in
    for _ in 0..1000 {
        connectome.step(&chemotaxis_neurons);
    }

    // Run  100 cycles of chemotaxis
    for _ in 0..1000 {
        connectome.step(&chemotaxis_neurons);
        connectome.discharge_query(MOTOR_NEURON_B, &mut motor_b_result);
        connectome.discharge_query(MOTOR_NEURON_A, &mut motor_a_result);
        print_motor_ab_discharges(&mut out_file, &motor_a_result, &motor_b_result)
            .map_err(|err| err.to_string())?;
    }

    for _ in 0..1000 {
        connectome.step(&nose_touch_neurons);
        connectome.discharge_query(MOTOR_NEURON_B, &mut motor_b_result);
        connectome.discharge_query(MOTOR_NEURON_A, &mut motor_a_result);
        print_motor_ab_discharges(&mut out_file, &motor_a_result, &motor_b_result)
            .map_err(|err| err.to_string())?;
    }
    Ok(())
}

fn print_motor_ab_discharges<W: Write>(
    mut w: W,
    a: &Vec<bool>,
    b: &Vec<bool>,
) -> Result<(), std::io::Error> {
    for i in 0..a.len() {
        write!(w, "{} ", if a[i] { 1 } else { 0 })?;
    }

    for i in 0..b.len() - 1 {
        write!(w, "{} ", if b[i] { 1 } else { 0 })?;
    }

    writeln!(w, "{}", if b[b.len() - 1] { 1 } else { 0 })?;
    Ok(())
}
