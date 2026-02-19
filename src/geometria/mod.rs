//definizione della strcut quadrato
pub struct Quadrato{
    lato: Option<f64>,
    diagonale: Option<f64>,
    area: Oprion<f64>
}
//definizione dei metodi relativi alla struct quadrato
impl Quadrato{
    pub fn new(l: Option<f64>,d: Option<f64>,a: Option<f64>) -> Self{
        Self {lato: l, diagonale: d, area: a}
    }

    pub fn calcola_area(&self){
        self.area = self.lato * self.lato;
    }

    pub fn calcola_diagonale(&self){
        self.diagonale = (self.lato*self.lato + self.lato*self.lato).sqrt();
    }
}
//definizione della struct rettangolo
pub struct Rettangolo{
    lato1: Option<f64>,
    lato2: Option<f64>,
    diagonale: Option<f64>,
    area: Option<f64>
}