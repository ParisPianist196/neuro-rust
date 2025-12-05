mod first_order_li;
use first_order_li::FirstOrderLi;

mod first_order_synapse;
use first_order_synapse::FirstOrderSynapse;

mod utils;
use utils::*;

use plotly::common::{DashType::Dot, Line, Mode};

use plotly::{Plot, Scatter};

fn full_plot(i: f64) -> Result<(), Box<dyn std::error::Error>> {
    const T_STEP: f64 = 0.001;
    let duration = 20.0;
    let times: Vec<f64> = (1..)
        .map(|n| n as f64 * T_STEP)
        .take_while(|&t| t < duration)
        .collect();

    let mut lif = FirstOrderLi::default();
    lif.tau_ref = 0.2;

    let mut i_history: Vec<f64> = [].to_vec();

    for _ in &times {
        // i = square_wave_at_t(t, 1, 1);

        lif.step(i, T_STEP);
        i_history.push(i);
    }

    let mut plot = Plot::new();
    lif.add_output_history_to_plot(&times, &mut plot, true);
    lif.add_spikes_to_plot(&times, &mut plot)?;
    lif.add_v_history_to_plot(&times, &mut plot);
    lif.add_v_th_history_to_plot(&times, &mut plot);

    let tracei = Scatter::new(times.clone(), i_history)
        .mode(Mode::Lines) // show as a line
        .line(Line::new().color("gray").width(2.0).dash(Dot))
        .name("I");
    plot.add_trace(tracei);

    plot.write_html("out.html");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    full_plot(1.001)?;
    Ok(())
}
