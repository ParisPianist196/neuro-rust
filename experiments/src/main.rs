mod connectome;
pub mod emulations;
pub mod lify_stuff;
use crate::emulations::c_elegans::c_elegans_nematode::test;

pub fn main() -> Result<(), String> {
    Ok(test()?)
}
