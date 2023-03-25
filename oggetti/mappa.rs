//volendo togliere mappa ed usare un Campo
pub mod mappa{
use crate::oggetti::{punto::punto::Punto, regno::regno::Regno};
pub struct Mappa {
    pub p1: Punto,
    pub p2: Punto,
    pub campi:Vec<Regno>,
}
 impl Mappa {
    pub fn area(&self) -> u16 {
        23
    }
}
}
