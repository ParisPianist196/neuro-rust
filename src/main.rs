fn potential(v_t: f64) -> f64 {
    let tau = 0.04;
    let energy_t = 1.0;
    (1.0/tau * energy_t) + (1.0/tau * v_t)
}

fn main() {

}
