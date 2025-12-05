mod first_order_li;
use first_order_li::FirstOrderLi;

mod utils;
use plotly::{
    common::{DashType::Dot, Line, Mode},
    layout::{Shape, ShapeLine},
};
use utils::*;

use plotly::{Layout, Plot, Scatter};

fn full_plot(mut i: f64) {
    const T_STEP: f64 = 0.001;
    let duration = 20.0;
    let times: Vec<f64> = (1..)
        .map(|n| n as f64 * T_STEP)
        .take_while(|&t| t < duration)
        .collect();

    let mut lif = FirstOrderLi::default();
    lif.tau_ref = 0.2;

    let mut v_history: Vec<f64> = [].to_vec();
    let mut i_history: Vec<f64> = [].to_vec();
    let mut spike_history: Vec<f64> = [].to_vec();
    let mut vth_history: Vec<f64> = [].to_vec();

    for t in &times {
        // i = square_wave_at_t(t, 1, 1);

        lif.step(i, T_STEP);
        i_history.push(i);
        v_history.push(lif.v);
        vth_history.push(lif.v_th);

        if lif.output > 0.0 {
            spike_history.push(*t);
        }
    }

    let mut plot = Plot::new();
    let tracev = Scatter::new(times.clone(), v_history);
    let tracei = Scatter::new(times.clone(), i_history)
        .mode(Mode::Lines) // show as a line
        .line(Line::new().color("gray").width(2.0).dash(Dot));
    let tracevth = Scatter::new(times.clone(), vth_history)
        .mode(Mode::Lines) // show as a line
        .line(Line::new().color("green").width(2.0).dash(Dot));

    plot.add_trace(tracev);
    plot.add_trace(tracei);
    plot.add_trace(tracevth);
    let vlines: Vec<Shape> = spike_history
        .iter()
        .map(|&spike_time| {
            Shape::new()
                .x0(spike_time)
                .x1(spike_time)
                .y0(0.0) // adjust y-range as needed
                .y1(1.0)
                .line(ShapeLine::new().color("red").width(1.0))
        })
        .collect();

    let layout = Layout::new().shapes(vlines);
    plot.set_layout(layout);
    plot.write_html("out.html");
}

fn main() {
    full_plot(1.001);
}
