mod first_order_lif;
use first_order_lif::FirstOrderLif;

mod first_order_synapse;
use first_order_synapse::FirstOrderSynapse;

mod first_order_lif_collection;
use first_order_lif_collection::FirstOrderLifCollection;

mod first_order_synapses_collection;
use first_order_synapses_collection::FirstOrderSynapsesCollection;

mod history;
mod simulation;
mod waveforms;
use nalgebra::DVector;
use waveforms::Sine;

mod utils;

use plotly::{Plot, Scatter};

use crate::{
    simulation::ExperimentSimulation,
    utils::{compute_tuning_curve, min_max_step_range},
    waveforms::PlotWaveform,
};

fn plot_synapse_example() -> Result<(), Box<dyn std::error::Error>> {
    const T_STEP: f64 = 0.01;
    let duration = 12.0;
    let num_steps = (duration / T_STEP) as usize;
    let times: Vec<f64> = (0..num_steps).map(|n| n as f64 * T_STEP).collect();

    // Sample rate should 1/T_STEP samples for each half of the duration
    let waveform: Sine = Sine::new(duration as f32 / 2. / T_STEP as f32, 1., 2., 0.);
    let input = waveform.get_samples(times.len());

    // Neuron
    let mut neuron = FirstOrderLif::new(300., 0.5, 1);

    // Synapse
    let mut synapse = FirstOrderSynapse::default();
    synapse.tau_s = 0.15;

    // Run simulation
    for idx in 0..times.len() {
        neuron.step(input[idx], T_STEP, None);
        // synapse.step(neuron.output, T_STEP);
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
    // synapse.add_decoded_output_to_plot(&times, phi, &mut plot);

    plot.write_html("out.html");

    Ok(())
}

fn plot_neuron_tuning_curves(collection: &mut FirstOrderLifCollection) -> Result<(), String> {
    const T_STEP: f64 = 0.01;
    let mut plt: Plot = Plot::new();
    let num_steps = (200. / T_STEP) as usize;
    let inputs: Vec<f64> = (0..num_steps).map(|n| n as f64 * T_STEP).collect();
    let mut curve: Vec<f64>;
    for n in collection.neurons.iter_mut() {
        curve = compute_tuning_curve(n, &inputs, 1., T_STEP)?;
        let trace = Scatter::new(inputs.clone(), curve);
        plt.add_trace(trace);
    }
    plt.write_html("out.html");
    Ok(())
}

fn plot_collection_tuning_curves(collection: &mut FirstOrderLifCollection) {
    const T_STEP: f64 = 0.01;
    let num_steps = (2.0 / T_STEP) as usize + 1; // +1 to include endpoint 1.0
    let inputs: Vec<f64> = (0..num_steps).map(|i| -1.0 + i as f64 * T_STEP).collect();
    let mut plt: Plot = Plot::new();

    for n in collection.neurons.iter() {
        let curve: Vec<f64> = inputs
            .iter()
            .map(|&input| n.analytical_rate(input * n.gain * n.encoder as f64 + n.bias))
            .collect();
        let trace = Scatter::new(inputs.clone(), curve);
        plt.add_trace(trace);
    }
    plt.write_html("out.html");
}

fn plot_synapses_signal() -> Result<(), Box<dyn std::error::Error>> {
    const T_STEP: f64 = 0.001;
    const DURATION: f64 = 12.;
    const NUM_NEURONS: usize = 15;
    let mut lifs = FirstOrderLifCollection::new(
        NUM_NEURONS,
        0.02,
        0.002,
        (25., 100.),
        (-1., 1.),
        vec![-1, 1],
    )?;
    let mut synapses = FirstOrderSynapsesCollection::new(15, 0.05);

    let times = min_max_step_range(0., DURATION, T_STEP);
    let mut plt: Plot = Plot::new();
    let waveform: Sine = Sine::new(DURATION as f32 / 2. / T_STEP as f32, 1., 2., 0.);
    let inputs = waveform.get_samples((DURATION / T_STEP) as usize);

    let decoders = lifs.get_decoders(&inputs)?;

    let mut exp_simulation = ExperimentSimulation::new(NUM_NEURONS, NUM_NEURONS);

    for i in inputs {
        let lifs_out = lifs.step(i, T_STEP, Some(&mut exp_simulation.neurons));
        synapses.step(&lifs_out, T_STEP, Some(&mut exp_simulation.synapses))?;
    }

    let phi_vec = DVector::from_vec(decoders);
    let outputs = exp_simulation.synapse_outputs()?;

    let decoded: DVector<f64> = outputs * phi_vec;

    let trace = Scatter::new(times.clone(), decoded.as_slice().to_vec());
    plt.add_trace(trace);
    plt.show();

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    plot_synapses_signal()?;
    Ok(())
}
