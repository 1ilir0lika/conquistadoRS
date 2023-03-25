pub mod campo {
    use std::usize;
    //use rand::thread_rng;
    use crate::functions::functions::area;
    use crate::functions::functions::concide;
    use crate::oggetti::mappa::mappa::Mappa;
    use crate::oggetti::player::player::Player;
    use crate::oggetti::punto::punto::Punto;
    use crate::oggetti::regno::regno::Regno;
    use rand::Rng;
    #[derive(Debug, Clone, PartialEq)]
    pub struct Campo(pub Vec<Punto>);
    impl Campo {
        pub fn area(&mut self) -> u16 {
            //calcolare area del campo,in generale di poligono del quale non si conosce solo le coordinate dei vertici
            //lati solo consecutivi e non ci possono essere lati diagonali tutti i punti devono tra di loro condividere a coppia una cordinata
            //rimuovere estremi che sono presenti nel vettore che costituisce il campo : se esiste min x min y elimino quello,invece se lo devo inventare con le altre coordinate lo aggiungo
            //questo discorso vale anche per gli estremi misti,nel senso con una componente min e l'altra max e viceversa
            let estremi = self.clone().extremes();
            let areatot = area(estremi[0], estremi[1]);
            //farlo fare ad un funzione
            self.0.retain(|punto| {
                if punto.uguale(&estremi[0])
                    || punto.uguale(&estremi[1])
                    || punto.uguale(&estremi[2])
                    || punto.uguale(&estremi[3])
                {
                    false
                } else {
                    true
                }
            });
            //calcolare area vuota,ovvero l'area dei punti rimanenti collegati per il vertice immaginario
            //cercare punto più vicino con un field(x o y) uguale,cercarne un altro rispetto a quello,calcolare l'area tra il primo e l'ultimo punt,eliminare i 2 punti usati,passare a quelli successivi
            let mut areavuota = 0;
            while self.0.len() > 2 {
                //primo e terzo punto sono gli estremi
                let mut p1 = &self.0[0].clone();
                self.0.swap_remove(0);
                let p2 = p1.closest(Campo(self.0.clone())).unwrap();
                let index = self.0.iter().position(|x| *x == p2).unwrap();
                self.0.remove(index);
                //fixare in caso p3 è uguale a None
                //P1=> p2 => nulla
                //Quindi p3 <= p1 => p2
                //P3 diverso da p2
                //P2 => p1 => p3
                //Quindi p2 diventano p1
                match p2.closest(Campo(self.0.clone())) {
                    Some(_x) => {
                        let p3 = p2.closest(Campo(self.0.clone())).unwrap();
                        println!("{:?} {:?} {:?}", p1, p2, p3);
                        areavuota += Campo(vec![*p1, p2, p3]).areatot();
                    }
                    None => {
                        p1 = &p2;
                        let realp2 = p1.closest(Campo(self.0.clone())).unwrap();
                        let p3 = realp2.closest(Campo(self.0.clone())).unwrap();
                        println!("{:?} {:?} {:?}", p1, realp2, p3);
                        areavuota += Campo(vec![*p1, realp2, p3]).areatot();
                    }
                }
            }
            println!("{}", areavuota);
            areatot - areavuota
        }
        pub fn areatot(self) -> u16 {
            let estremi = self.extremes();
            area(estremi[0], estremi[1])
        }
        //ritorna Mappa con il vettore di regni
        //è un const generic perchè altrimenti n non era const e quindi non potevo fare l'array di quella lunghezza
        pub fn dividi<const N: usize>(mut self, mut players: [Player; N]) -> () {
            //definisci per ogni player il suo regno e quindi il suo territorio
            let area: i16 = self.area() as i16 / N as i16;
            let estremi = self.extremes();
            let mut rng = rand::thread_rng();
            #[macro_export]
            macro_rules! gencampo {
                ($i:expr) => {
                    Campo(vec![
                        estremi[$i],
                        Punto {
                            x: estremi[$i].x,
                            y: rng.gen_range(estremi[0].y..estremi[1].y),
                        },
                        Punto {
                            x: (area
                                / (players[$i].regno.campo.0[1].x
                                    - players[$i].regno.campo.0[0].x)
                                    .abs()),
                            y: players[$i].regno.campo.0[1].y,
                        },
                        Punto {
                            x: players[$i].regno.campo.0[2].x,
                            y: players[$i].regno.campo.0[0].y,
                        },
                    ])
                };
            }
            for i in 0..N {
                if i <= 4 {
                    //regno dei 4 estremi,devono essere rettangoli,altrimenti vengono attaccati da troppe persone
                    //fare controllo che non si sovrappongano prima assegnare questo campo ad un campo,poi se non si sovrappone con alcun regno che itero per i fino a quello attuale assegnarlo
                    //fare solo se i > 1
                    loop {
                        let campo: Campo = gencampo!(i);
                        if concide(players[i - 1].regno.campo.clone(), campo.clone()) {
                        } else {
                            players[i].regno.campo = campo;
                            break;
                        }
                    }
                }
                //generare regni normali
                //appunti sul diario infondo

                //generare ultimo regno
                //if i=N-1{
                //in questo caso non usare random ma dare tutti i punti non usati
                //}
            }
        }
        pub fn extremes(self) -> [Punto; 4] {
            let mut max_x: i16 = self.0[0].x;
            let mut max_y: i16 = self.0[0].y;
            let mut min_x: i16 = self.0[0].x;
            let mut min_y: i16 = self.0[0].y;
            for coord in self.0.iter() {
                if coord.x > max_x {
                    max_x = coord.x;
                } else if coord.x < min_x {
                    min_x = coord.x;
                }
                if coord.y > max_y {
                    max_y = coord.y;
                } else if coord.y < min_y {
                    min_y = coord.y;
                }
            }
            let estrem1 = Punto { x: max_x, y: max_y };
            let estrem2 = Punto { x: min_x, y: min_y };
            let estrem3 = Punto { x: max_x, y: min_y };
            let estrem4 = Punto { x: min_x, y: max_y };
            [estrem1, estrem2, estrem3, estrem4]
        }
//        pub fn coincideprecedenti(&self,i:u8) -> bool{
//controlla se campo coincide con quelli precedenti
//        }
    }
}
