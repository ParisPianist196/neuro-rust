pub fn sine_wave() {
    let waveform = wf!(
        f64,
        200.,
        sine!(frequency: 100., amplitude: 10.),
        dc_bias!(20.)
    );
}
