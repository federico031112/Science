

pub struct Vettore{
    x: f64,
    y: f64,
}
//descrizione di un vettore
impl Vettore {
    //metodo costruttore
    pub fn new(dx: f64, up: f64) -> Self {
        Self { x: dx, y: up}
    }
    //funzione che calcola il modulo di un vettore
    pub fn modulo(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    //funzione che calcola la somma di due vettori
    pub fn somma(&self,vec: &Vettore) -> Vettore{
        let xres = self.x + vec.x;
        let yres = self.y + vec.y;
        Vettore::new(xres,yres)
    }
    //funzione che calcola il prodotto tra uno scalare ed un vettore
    pub fn prodotto_scalare_vettore(&self,n: f64) -> Vettore {
        Vettore::new(n*self.x,n*self.y)
    }
    //funzione che permette di calcolare il prodotto scalare tra due vettori prende in input due vettori e l'angolo compreso tra essi misurato in radianti
    pub fn prodotto_scalare_tra_vettori(&self,vec: Vettore,angolo: f64) -> f64 {
        self.modulo()*vec.modulo()*angolo
    }

    pub fn prodotto_vettoriale_tra_vettori(&self, vec: &Vettore) -> Vettore3D {
        let z: f64 = (self.x*vec.y)-(self.y*vec.x);
        Vettore3D { x: 0.0, y: 0.0, z }
    }
    //funzione che calcola il versore di un vettore
    pub fn versore(&self) -> Vettore {
        Vettore::new(self.x/self.modulo(),self.y/self.modulo())
    }

}
//funzione che prende in input il modulo di un vettore e l'angolo ad esso associato una volta traslato all origine,
//e permette di descrivere questo vettore tramite le sue componenti x e y restituendo un vettore
pub fn calcola_componenti_vettore(modulo: f64,angolo_rad: f64) -> Vettore{
    let x = modulo * angolo_rad.cos();
    let y = modulo * angolo_rad.sin();
    let vec: Vettore = Vettore::new(x, y);
    vec
}


//calcola la velocità media in metri/secondi
pub fn velocità_media(s1: &Vettore,s2 :&Vettore, t1: f64, t2: f64) -> Vettore {
    let v = Vettore::new((s2.x-s1.x)/(t2-t1),(s2.y-s1.y)/(t2-t1));
    v
}

pub struct Vettore3D {
    x: f64,
    y: f64,
    z: f64
}

impl Vettore3D {
    pub fn new(&self,x: f64,y: f64,z: f64) -> Vettore3D{
        Self { x, y, z }
    }
}