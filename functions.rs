pub mod functions{
use std::cmp::Ordering;
use crate::oggetti::{punto::punto::Punto, campo::campo::Campo};
    pub fn area(p1: Punto,p2: Punto) -> u16 {
    let Punto { x: x1, y: y1 } = p1;
    let Punto { x: x2, y: y2 } = p2;
    let area: i16 = (x2 - x1) * (y2 - y1);
    return {
        match (area).cmp(&0) {
            Ordering::Less => (area * -1).try_into().unwrap(),
            Ordering::Equal => 0,
            Ordering::Greater => area.try_into().unwrap(),
        }
    };
}
//questo funziona solo quando gli estremi sono anche  dei punti,quindi devono essere per forza dei rettangolo senzqa spazi vuoti,altrimenti potrebbe segnare vero anche quando non lo è
//per figure irregolare dovrei verificare quelle proprietà per ogni punto,ma siccome sta funzione la uso solo per i primi quattro quadranti iniziali va bene
    pub fn concide(c1:Campo,c2:Campo)->bool{
        let estrem1=c1.extremes();
        let estrem2=c2.extremes();
        //controllare che somma area effettiva dei 2 campi sia minore di area trai 2 estremi dei due campi
        //altrimenti verificare che ogni estremo di c1 non abbia delle coordinate comprese tra i 2 estremi,quindi estremo max ed estremo min
        for punto1 in estrem1{
            if punto1.x < estrem2[0].x && punto1.x > estrem2[1].x && punto1.y < estrem2[0].y && punto1.y > estrem2[1].y{
                return true;
            }
        }
        false   
    }
}
