pub mod banca{
    use crate::oggetti::regno::regno::Comprabile;
    use crate::traits::buyable::buyable::Buyable;
    use crate::Punto;
    pub const PRICE0:u16=500;
    pub const PRICE1:u16=800;
    pub const PRICE2:u16=1500;
    pub const PRICE3:u16=2000;
    pub const PRICE4:u16=5000;
    pub const TIME0:u16=1000;
    pub const TIME1:u16=1000;
    pub const TIME2:u16=1000;
    pub const TIME3:u16=1000;
    pub const TIME4:u16=1000;
    pub const SOLDI0:u8=100;
    pub const SOLDI1:u8=100;
    pub const SOLDI2:u8=100;
    pub const SOLDI3:u8=100;
    pub const SOLDI4:u8=100;
#[derive(Clone, Copy)]
    pub struct Banca{
        //soldi fatti ogni tempo
        pub soldi:u8,
        //millisecondi
        pub tempo:u16,
        pub posizione:Punto,
        //livello che si può potenziare
        pub livello:u8,
    }
    impl Buyable for Banca{
        fn prezzo(&self,livello:Option<u8>)->u16 {
            match livello{
                Some(0)=>PRICE0,
                Some(1) =>PRICE1,
                Some(2) =>PRICE2,
                Some(3) =>PRICE3,
                _ => PRICE4,
            }
        }
        fn build(&self,punto:Punto,livello:Option<u8>)->Comprabile{
            match  livello{
                Some(0)=>    
                    Comprabile::Banca(Banca { soldi: (SOLDI0), tempo: (TIME0), posizione: (punto), livello: (livello.unwrap()) })
                ,
                Some(1)=>    
                    Comprabile::Banca(Banca { soldi: (SOLDI1), tempo: (TIME1), posizione: (punto), livello: (livello.unwrap()) })
                ,
                Some(2)=>    
                    Comprabile::Banca(Banca { soldi: (SOLDI2), tempo: (TIME2), posizione: (punto), livello: (livello.unwrap()) })
                ,       
                Some(3)=>    
                    Comprabile::Banca(Banca { soldi: (SOLDI3), tempo: (TIME3), posizione: (punto), livello: (livello.unwrap()) })
                ,
                _=>    
                    Comprabile::Banca(Banca { soldi: (SOLDI4), tempo: (TIME4), posizione: (punto), livello: (livello.unwrap()) })
                ,

            }
        }
    }
      
}