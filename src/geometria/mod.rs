//definizione della strcut quadrato
pub struct Quadrato{
    pub lato: f64,
    pub diagonale: f64,
    pub area: f64,
    pub perimetro: f64
}
//definizione dei metodi relativi alla struct quadrato
impl Quadrato{
    pub fn new(l: f64,d: f64,a: f64, p: f64) -> Self{
        Self {lato: l, diagonale: d, area: a, perimetro: p}
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

    pub fn calcola_perimetro(&mut self){
        self.perimetro = self.lato * 4.0;
    }
}
//definizione della struct rettangolo
pub struct Rettangolo{
    lato1: f64,
    lato2: f64,
    diagonale: f64,
    area: f64,
    perimetro: f64
}

impl Rettangolo{
    pub fn new(l1: f64, l2: f64, d: f64, a: f64, p: f64) -> Self{
        Self { lato1: l1, lato2: l2, diagonale: d, area: a, perimetro: p }
    }

    pub fn calcola_area(&mut self){
        self.area = self.lato1*self.lato2;
    }

    pub fn calcola_diagonale(&mut self){
        self.diagonale = (self.lato1*self.lato1 + self.lato2*self.lato2).sqrt();
    }

    pub fn calcola_perimetro(&mut self){
        self.perimetro = self.lato1*2.0 + self.lato2*2.0;
    }
}

pub struct Cerchio{
    raggio: f64,
    perimetro: f64,
    area: f64
}

impl Cerchio{
    pub fn new(r: f64, p: f64, a: f64) -> Self{
        Self {raggio: r, perimetro: p, area: a}
    }

    pub fn calcola_area(&mut self){
        self.area = self.raggio*self.raggio*3.14;

    }

    pub fn calcola_perimetro(&mut self){
        self.perimetro = 2.0*3.14*self.raggio;
    }
}

//consiglio per la gestione di un triangolo i dati possono essere complessi da gestire, meglio avere a fianco un disegno che rappresenti il triangolo
pub struct Triangolo{
    l1: f64,
    l2: f64,
    l3: f64,
    alfa: f64,
    beta: f64,
    gamma: f64
}

impl Triangolo{
    pub fn new(lato1: f64, lato2: f64, lato3: f64) -> Self {
        Self { l1: lato1, l2: lato2, l3: lato3, alfa: 0.0, beta: 0.0, gamma: 0.0 }
    }

    
}


