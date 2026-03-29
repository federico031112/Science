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
/*per risolvere un triangolo qualsiasi bisogna essere a conoscenza di:
    1. due angoli e un lato
    2. tre lati
    3. due lati e l'angolo compreso
    4. due lati e l'angolo opposto ad uno di essi */
pub struct Triangolo{
    l1: f64,
    l2: f64,
    l3: f64,
    alfa: f64,
    beta: f64,
    gamma: f64,
    perimetro: f64,
    area: f64
}

impl Triangolo{
    pub fn new(lato1: f64, lato2: f64, lato3: f64,a: f64, b: f64, c:f64, per: f64, area: f64) -> Self {
        Self { l1: lato1, l2: lato2, l3: lato3, alfa: a, beta: b, gamma: c, perimetro: per, area: area }
    }

    pub fn set_l1(&mut self, la1: f64){
        self.l1 = la1;
    }

    pub fn set_l2(&mut self, l2: f64){
        self.l2 = l2;
    }

    pub fn set_l3(&mut self, l3: f64){
        self.l3 = l3;
    }

    pub fn set_alfa(&mut self, a: f64){
        self.alfa = a;
    }

    pub fn set_beta(&mut self, b: f64){
        self.beta = b;
    }

    pub fn set_gamma(&mut self, g: f64){
        self.gamma = g;
    }

    pub fn calcola_perimetro(&mut self) {
        self.perimetro = self.l1 + self.l2 + self.l3;
    }

    pub fn calcola_area(&mut self) {
        self.area = 0.5 * self.l1 * self.l2 * self.alfa.cos();
    }


    
}


//da vedere le formule goniometriche per risolvere un triangolo qualsiasi
pub fn teorema_del_coseno(l1: f64, l2: f64, alfa: f64) -> f64 {
    (l1.powf(2.0)+l2.powf(2.0) - 2.0*l1*l2*alfa.cos()).sqrt()
}

pub fn teorema_dei_seni(l1: f64, l2: f64, alfa: f64) -> f64 {
    (alfa.sin()*l2/l1).asin()
}

pub enum Forma2D {
    Rettangolo(&Rettangolo),
    Quadrato(&Quadrato),
    Triangolo(&Triangolo),
    Cerchio(&Cerchio)
}

pub struct Prisma {
    base: Forma2D,
    altezza: f64,
    volume: f64
}

impl Prisma {
    fn new(base: Forma2D, altezza: f64, volume: f64) -> Self{
        Self { base, altezza, volume }
    }

    fn calcola_area(&mut self) {
        match self.base{
            Forma2D::Rettangolo(r) => self.volume = r.area * self.altezza,
            Forma2D::Quadrato(q) => self.volume = q.area * self.altezza,
            Forma2D::Triangolo(t) => self.volume = t.area * self.altezza,
            Forma2D::Cerchio(c) => self.volume = c.area * self.altezza,
            _ => self.volume = 0.0
        }
    }
}


