#[derive(Default, Debug)]
pub struct History {
    data: Vec<f64>,
}

impl History {
    pub fn record(&mut self, x: f64) {
        self.data.push(x);
    }

    pub fn get(&self) -> &[f64] {
        &self.data
    }
}

pub trait HasHistories {
    /// Return a list of (&str, &History) for plotting and inspection.
    fn histories(&self) -> Vec<(&str, &History)>;
}
