use std::fs::File;

use crate::connectome::Connectome;

pub fn test() -> Result<(), String> {
    let nose_touch_neurons = vec![
        "N_FLPR", "N_FLPL", "N_ASHL", "N_ASHR", "N_IL1VL", "N_IL1VR", "N_OLQDL", "N_OLQDR",
        "N_OLQVR", "N_OLQV",
    ];

    let chemotaxis_neurons = [
        "N_ADFL", "N_ADFR", "N_ASGR", "N_ASGL", "N_ASIL", "N_ASIR", "N_ASJR", "N_ASJL",
    ];

    let f = File::open("abc").map_err(|err| err.to_string())?;
    let mut connectome = Connectome::new(f, 12.)?;

    // Perform burn in
    for _ in 0..1000 {
        connectome.step(&chemotaxis_neurons);
    }

    // Run  100 cycles of each behavior
    for _ in 0..1000 {
        connectome.step(&chemotaxis_neurons);
    }
    Ok(())
}
