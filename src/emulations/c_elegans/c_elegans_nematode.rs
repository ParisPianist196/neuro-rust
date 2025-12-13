use std::fs::File;

use crate::connectome::Connectome;
use std::io::Write;

pub fn test() -> Result<(), String> {
    const THRESHOLD: f32 = 30.;

    pub const MOTOR_NEURON_B: &[&str] = &[
        "N_DB1", "N_DB2", "N_DB3", "N_DB4", "N_DB5", "N_DB6", "N_DB7", "N_VB1", "N_VB2", "N_VB3",
        "N_VB4", "N_VB5", "N_VB6", "N_VB7", "N_VB8", "N_VB9", "N_VB10", "N_VB11",
    ];

    pub const MOTOR_NEURON_A: &[&str] = &[
        "N_DA1", "N_DA2", "N_DA3", "N_DA4", "N_DA5", "N_DA6", "N_DA7", "N_DA8", "N_DA9", "N_VA1",
        "N_VA2", "N_VA3", "N_VA4", "N_VA5", "N_VA6", "N_VA7", "N_VA8", "N_VA9", "N_VA10", "N_VA11",
        "N_VA12",
    ];

    let nose_touch_neurons = vec![
        "N_FLPR", "N_FLPL", "N_ASHL", "N_ASHR", "N_IL1VL", "N_IL1VR", "N_OLQDL", "N_OLQDR",
        "N_OLQVR", "N_OLQV",
    ];

    let chemotaxis_neurons = [
        "N_ADFL", "N_ADFR", "N_ASGR", "N_ASGL", "N_ASIL", "N_ASIR", "N_ASJR", "N_ASJL",
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
        write!(w, "{} ", a[i])?;
    }

    for i in 0..b.len() - 1 {
        write!(w, "{} ", b[i])?;
    }

    writeln!(w, "{}", b[b.len() - 1])?;
    Ok(())
}
