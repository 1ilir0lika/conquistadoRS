pub mod regno{
        use crate::{traits::buyable::buyable::Buyable, oggetti::{campo::campo::Campo, punto::punto::Punto, clan::clan::Clan, caserma::caserma::Caserma, banca::banca::Banca}};
    #[derive(Clone)]
    pub struct Regno{
        pub campo:Campo,
        pub soldi:u16,
        pub capitale:Punto,
        pub area:u16,
        pub clan:Option<Clan>,
        pub soldati:u8,
        pub caserme:Vec<Caserma>,
        pub banche:Vec<Banca>,
    }
#[derive(Clone, Copy)]
    pub enum Comprabile{
        Banca(Banca),
        Caserma(Caserma),
    }
    impl Buyable for Comprabile{
         fn prezzo(&self,livello: Option<u8>)->u16 {
         match self{
            Self::Banca(x) =>{x.prezzo(livello)},
            Self::Caserma(x) =>{x.prezzo(livello)},
         }
            //se ricursione fixare come fatto sotto col match
        }
         fn build(&self,punto:Punto,livello: Option<u8>)->Comprabile {
               match self{
            Self::Banca(x) =>{x.build(punto, livello)},
            Self::Caserma(x) =>{x.build(punto, livello)},
         }
        }
    }
    impl Regno{
       pub fn updatearea(mut self){
            self.area=self.campo.area();
        }
       pub fn movecapital(mut self,newcapital:Punto){
            self.capitale=newcapital;
        }
        //per segnare che una funzione non ritorna nulla : !
        pub fn buy(mut self,oggetto: Comprabile,livello:Option<u8>,punto:Punto){
            self.soldi-=oggetto.prezzo(livello);
            //guardare enum comprabile
            match oggetto.build(punto, livello){
                //fare eventuali controlli su posizione(è tuo il terreno nel senso esistono dei punti tuoi più estremi di quello oppure è già usato il terreno)
                Comprabile::Banca(value)=>{
                                    self.banche.push(value);
                                 },
                Comprabile::Caserma(value)=>{
                                    self.caserme.push(value);
                }

            }
        } 
        pub fn sell(&mut self,oggetto:Comprabile,livello:Option<u8>)->u16{
            self.demolisci(oggetto);
            &self.soldi+oggetto.prezzo(livello)-200
        }
        pub fn demolisci(&mut self,oggetto: Comprabile){
                        match oggetto{
                Comprabile::Banca(value)=>{
                                    let index = self.banche.iter().position(|x| x.posizione.uguale(&value.posizione)).unwrap();
                                    self.banche.swap_remove(index);
                                 },
                Comprabile::Caserma(value)=>{
                                    let index = self.caserme.iter().position(|x| x.posizione.uguale(&value.posizione)).unwrap();
                                    self.caserme.swap_remove(index);
                }

            }
        }
        pub fn createclan(){

        }
        pub fn joinclan(){

        }
        pub fn leaveclan(){

        }
        //pub fn upgrade(self){
        // migliorare oggetto buyable pagando la differenza trai livelli e qualcosa in più    
        //fare trait upgradable e gestirlo dalla sua implementazione
        //}
        //fn asta(){
        //mettere all'asta un proprio possedimento agli altri giocatori
        //}
    }
}