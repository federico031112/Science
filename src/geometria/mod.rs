//definizione della strcut quadrato
pub struct Quadrato{
    pub lato: f64,
    pub diagonale: f64,
    pub area: f64
}
//definizione dei metodi relativi alla struct quadrato
impl Quadrato{
    pub fn new(l: f64,d: f64,a: f64) -> Self{
        Self {lato: l, diagonale: d, area: a}
    }

    pub fn calcola_area(&mut self){
        self.area = self.lato * self.lato;
    }

    pub fn calcola_diagonale(&mut self){
        self.diagonale = (self.lato*self.lato + self.lato*self.lato).sqrt();
    }

    pub fn calcola_lato_da_area(&mut self){
        self.lato = self.area.sqrt();
    }
}
//definizione della struct rettangolo
pub struct Rettangolo{
    lato1: Option<f64>,
    lato2: Option<f64>,
    diagonale: Option<f64>,
    area: Option<f64>
}