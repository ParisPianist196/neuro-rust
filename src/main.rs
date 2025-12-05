mod first_order_li;
use first_order_li::FirstOrderLi;

mod first_order_synapse;
use first_order_synapse::FirstOrderSynapse;

mod waveforms;
use waveforms::Sine;

mod utils;

use plotly::Plot;

use crate::waveforms::PlotWaveform;

fn full_plot() -> Result<(), Box<dyn std::error::Error>> {
    const T_STEP: f64 = 0.01;
    let duration = 12.0;
    let num_steps = (duration / T_STEP) as usize;
    let times: Vec<f64> = (0..num_steps).map(|n| n as f64 * T_STEP).collect();

    // Sample rate should 1/T_STEP samples for each half of the duration
    let waveform: Sine = Sine::new(duration as f32 / 2. / T_STEP as f32, 1., 2., 0.);
    let input = waveform.get_samples(times.len());

    // Neuron
    let mut neuron = FirstOrderLi::new(300., 0.5);

    // Synapse
    let mut synapse = FirstOrderSynapse::default();
    synapse.tau_s = 0.15;

    // Run simulation
    for idx in 0..times.len() {
        neuron.step(input[idx], T_STEP);
        synapse.step(neuron.output, T_STEP);
    }

    // Plot results
    let mut plot = Plot::new();
    waveform.add_to_plot(&times, &mut plot, "I");

    // neuron.add_output_history_to_plot(&times, &mut plot, true);
    // neuron.add_spikes_to_plot(&times, &mut plot)?;
    // neuron.add_v_history_to_plot(&times, &mut plot);
    // neuron.add_v_th_history_to_plot(&times, &mut plot);

    // synapse.add_output_history_to_plot(&times, &mut plot);

    // Decode output to determine input
    let phi = neuron.decoder(-2., 2., 0.1);
    synapse.add_decoded_output_to_plot(&times, phi, &mut plot);

    plot.write_html("out.html");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    full_plot()?;
    Ok(())
}
