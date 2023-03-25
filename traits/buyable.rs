pub mod buyable{
    use crate::oggetti::{punto::punto::Punto, regno::regno::Comprabile};

pub trait Buyable{
    fn prezzo(&self,livello: Option<u8>)->u16;
    fn build(&self,punto:Punto,livello: Option<u8>)->Comprabile;
}
}