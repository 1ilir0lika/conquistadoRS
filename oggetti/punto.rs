pub mod punto {
    use crate::oggetti::campo::campo::Campo;
    #[derive(Copy, Debug, Clone, PartialEq)]
    pub struct Punto {
        pub x: i16,
        pub y: i16,
    }
    impl Punto {
        //usare tratto eq o partialeq
        pub fn uguale(&self, punto: &Punto) -> bool {
            self.x == punto.x && self.y == punto.y
        }
        pub fn simile(&self, punto: &Punto) -> Option<char> {
            if punto.x == self.x {
                Some('x')
            } else if punto.y == self.y {
                Some('y')
            } else {
                None
            }
        }
        //più vicino con stessa x
        pub fn closestx(&self, punti: Campo) -> Option<Punto> {
            let mut closestby_x: Punto = punti.0[0];
            let mut closest_y = &punti.0[0].y;
            for k in 0..punti.0.len() {
                for n in k..(&punti).0.len() {
                    if &punti.0[n].x == &self.x {
                        //verifica che i punti siano i più vicini e quindi se x è uguale che la y sia la più vicina
                        if (punti.0[n].y - &self.y).abs() < (closest_y - &self.y).abs() {
                            closestby_x = punti.0[n];
                            closest_y = &punti.0[n].y;
                        }
                    }
                }
            }
            return Some(closestby_x);
        }
        //più vicino con stessa y
        pub fn closesty(&self, punti: Campo) -> Option<Punto> {
            let mut closestby_y = punti.0[0];
            let mut closest_x = &punti.0[0].x;
            for k in 0..punti.0.len() {
                for n in k..(&punti).0.len() {
                    if &punti.0[n].y == &self.y {
                        //verifica che i punti siano i più vicini e quindi se x è uguale che la y sia la più vicina
                        if (punti.0[n].x - &self.x).abs() < (closest_x - &self.x).abs() {
                            closestby_y = punti.0[n];
                            closest_x = &punti.0[n].x;
                        }
                    }
                }
            }
            return Some(closestby_y);
        }

        //verifica che non esista anche un valore che ha in comune l'altra coordinata è che è più vicino
        //implemenare per punti funzione che dica quale trai 2 punti sia il più vicino a quello self

        pub fn closer(self, pointx: Option<Punto>, pointy: Option<Punto>) -> Option<Punto> {
            let mut totaldiffx = 0;
            let mut totaldiffy = 0;
            match pointx {
                Some(_x) => {
                    {
                        match pointy {
                            Some(_x) => {
                                totaldiffx += (&self.x - pointx.unwrap().x).abs();
                                totaldiffx += (&self.y - pointx.unwrap().y).abs();
                                totaldiffy += (&self.x - pointy.unwrap().x).abs();
                                totaldiffy += (&self.y - pointy.unwrap().y).abs();
                                if totaldiffx <= totaldiffy {
                                    pointx
                                } else {
                                    pointy
                                }
                            }
                            None => return pointx,
                        }
                    }
                }
                None => match pointy {
                    Some(_x) => return pointy,
                    None => return None,
                },
            }
        }
        pub fn closest(self, punti: Campo) -> Option<Punto> {
            //  self.closer(Some(&self.closesty(punti)).unwrap().as_ref(),Some(&self.closestx(punti)).unwrap().as_ref())
            //  self.closer( Some(self.closesty(punti).unwrap(), )(self.closestx(punti))).unwrap();
            let closesty = self.closesty(punti.clone());
            let closestx = self.closestx(punti.clone());
            self.closer(closestx, closesty)
        }
    }
}
