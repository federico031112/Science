

pub fn equazione_primo_grado(a: f64, b: f64) -> f64{
    let x: f64 = (-b)/a;
    x
}
//enum per il ritorno di una equazione di secondo grado
pub enum SoluzioneEquazioneSecondoGrado{
    DueReali(f64, f64),
    UnaReale(f64),
    Complessa{ reale: f64, imm: f64},
    Nessuna
}
// funzione che risolve le ecquazioni di secondo grado
pub fn equazione_secondo_grado(a: f64, b: f64, c:f64) -> SoluzioneEquazioneSecondoGrado{
    let discriminante: f64 = (b*b)-(4.0*a*c);
    if a == 0.0{
        let result = equazione_primo_grado(b,c);
        SoluzioneEquazioneSecondoGrado::UnaReale(result)
    }else if discriminante > 0.0 {
        let sqrt_d = discriminante.sqrt();
        let q: f64 = if b >= 0.0{
            -0.5 + (b * sqrt_d)
        }else{
            -0.5 - (b * sqrt_d)
        };
        let x1 = q / a;
        let x2 = c / q;
        SoluzioneEquazioneSecondoGrado::DueReali(x1,x2)
    } else if discriminante == 0.0{
        let result = (-b)/(2.0*a);
        SoluzioneEquazioneSecondoGrado::UnaReale(result)
    } else if discriminante < 0.0 {
        let reale = (-b)/(2.0*a);
        let imm = ((-discriminante).sqrt())/(2.0*a);
        SoluzioneEquazioneSecondoGrado::Complessa{reale: reale, imm: imm}
    } else {
        SoluzioneEquazioneSecondoGrado::Nessuna
    }
}
//funzione che calcola la sommatoria di alcuni numeri, prende in input un'array oppure una lista
pub fn sommatoria(numeri: &[f64]) -> f64{
    let mut somma: f64 = 0.0;
    for &n in numeri {
        somma = somma + n;
    }
    somma 
}
//funzione che calcola il fattoriale di un numero, prende in input un numero positivo con massimo 32 bit cioè compreso tra: 0 e circa 4 miliardi
pub fn fattoriale(n: u32) -> u32{
    let mut fat: u32 = 1;
    for i in 1..=n{
        fat = fat * i;
    }
    fat
}

//funzione che trasforma i simboli numerici normali in apici
fn trasforma_in_apice(num: f64) -> String{
    num.to_string().chars().map(|c|{
        match c {
            '0' => '⁰', '1' => '¹', '2' => '²', '3' => '³', '4' => '⁴',
            '5' => '⁵', '6' => '⁶', '7' => '⁷', '8' => '⁸', '9' => '⁹',
            '+' => '⁺', '-' => '⁻', '=' => '⁼', '(' => '⁽', ')' => '⁾',
            '.' | ',' => '˙', // Il punto decimale non ha un apice perfetto in Unicode
            'n' => 'ⁿ',
            _ => c,
        }
    }).collect()
}
//definizione della struct potenza e della sua implementazione
pub struct Potenza{
    esponente: f64,
    coefficiente: f64,
}

//definizione dei metodi potenza
impl Potenza{
    pub fn new(coeff: f64, esp: f64) -> Self {
        Self { esponente: esp, coefficiente: coeff }
    }

    pub fn print(&self) -> String{
        format!("{}x{}",self.coefficiente,trasforma_in_apice(self.esponente))
    }

    pub fn derivata(&self) -> Potenza{
        let derivata: Potenza = Potenza::new(self.coefficiente*self.esponente, self.esponente-1.0);
        derivata
    }

    pub fn calcola(&self,base: f64) -> Result<f64, String>{
        if base > 0.0 {
            Ok(base.powf(self.esponente) * self.coefficiente)
        }else{
            Err("base negativa".to_string())
            //introdurre la gestione della base negativa tramite l'impiego dei numeri complessi
        }
    }
}

//definizione della struct seno e della sua implementazione, il risultato è in radianti non gradi
pub struct Seno{
    ampiezza: f64,
    frequenza_angolare: f64,
    fase: f64,
    //potrei implementare la traslazione verticale della funzione seno
}

//definizione dei metodi seno
impl Seno {
    pub fn new(amp: f64,freq: f64,fas: f64) -> Self {
        Self { ampiezza: amp, frequenza_angolare: freq, fase: fas }
    }

    pub fn calcola(&self, x: f64) -> f64 {
        self.ampiezza * (self.frequenza_angolare * (x + self.fase)).sin()
    }

    pub fn print(&self) -> String{
        format!("{}sin({}(x+({})))",self.ampiezza,self.frequenza_angolare,self.fase)
    }

    pub fn derivata(&self) -> Coseno{
        Coseno::new(self.ampiezza*self.frequenza_angolare, self.frequenza_angolare, self.fase)
    }
}

//definizione della struct coseno, il risultato è in radianti non gradi
pub struct Coseno{
    ampiezza: f64,
    frequenza_angolare: f64,
    fase: f64,
    //potrei implementare la traslazione verticale della funzione coseno
}

//definizione dei metodi della funzione coseno
impl Coseno{
    pub fn new(amp: f64,freq: f64,fas: f64) -> Self {
        Self {ampiezza: amp, frequenza_angolare: freq, fase: fas }
    }

    pub fn calcola(&self, x: f64) -> f64{
        self.ampiezza * (self.frequenza_angolare*(self.fase+x)).cos()
    }

    pub fn print(&self) -> String{
        format!("{}cos({}(x+({}))",self.ampiezza.to_string(),self.frequenza_angolare.to_string(),self.fase.to_string())
    }

    pub fn derivata(&self) -> Seno{
        Seno::new(-(self.ampiezza*self.frequenza_angolare), self.frequenza_angolare, self.fase)
    }
}

//funzione per convertire i gradi in radianti
pub fn converti_gradi_radianti(gradi: f64) -> f64{
    gradi*3.14/180.0
}

//funzione per convertire i radianti in gradi
pub fn converti_radianti_gradi(radianti: f64) -> f64{
    180.0*radianti/3.14
}
//definizione di matrice a n e m dimensioni
pub struct Matrice{
    pub righe: usize,
    pub colonne: usize,
    pub data: Vec<Vec<f64>>
}
//funzioni legate alla matrice
impl Matrice{
    pub fn new( righe: usize, colonne: usize, data: Vec<Vec<f64>>) -> Result<Self, String>{
        if data.len() == righe {
            let mut count = 0;
            for i in 0..righe {
                if data[i].len() == colonne{
                    count+=1;
                }else{
                    break;
                }
            }
            if count == righe {
                Ok(Self { righe, colonne, data })
            }else{
                Err("errore".to_string())
            }
        }else{
            Err("errore".to_string())
        }
        
    }
    //funzione per il calcolo del determinante di una matrice
    pub fn calcola_determinante(&self) -> Result<f64, String>{
        if self.righe == self.colonne{
            match self.righe {
                0 => Err("matrice vuota".to_string()),
                1 => Ok(self.data[0][0]),
                2 => Ok(self.data[0][0]*self.data[1][1]-self.data[0][1]*self.data[1][0]),
                3 => Ok(self.data[0][0]*self.data[1][1]*self.data[2][2]+
                    self.data[0][1]*self.data[1][2]*self.data[2][0]+
                    self.data[0][2]*self.data[1][0]*self.data[2][1]-
                    self.data[0][2]*self.data[1][1]*self.data[2][0]-
                    self.data[0][0]*self.data[1][2]*self.data[2][1]-
                    self.data[0][1]*self.data[1][0]*self.data[2][2]),
                _ => self.determinante_gauss()
            }
        }else{
            Err("matrice non quadrata".to_string())
        }
    }
        //funzione da capire meglio la sua logica commentandola
        fn determinante_gauss(&self) -> Result<f64, String> {
        let n = self.righe;
        let mut a = self.data.clone();
        let mut det = 1.0;
        let mut scambi = 0;
        let epsilon = 1e-10;
        
        for i in 0..n {
            // Pivoting parziale
            let mut pivot = i;
            let mut max_val = a[i][i].abs();
            //calcolo il valore massimo della diagonale della matrice e se necessario scambio i valori
            for k in i+1..n {
                if a[k][i].abs() > max_val {
                    max_val = a[k][i].abs();
                    pivot = k;
                }
            }
            //controllo che il valore non sia prossimo allo zero, in tal caso ritorno una matrice singolare
            if max_val < epsilon {
                return Ok(0.0);  // Matrice singolare
            }
            //verifico se ho effettuato degli scambi tra i valori
            if pivot != i {
                a.swap(i, pivot);
                scambi += 1;
            }
            //calcolo il determinante
            det *= a[i][i];
            
            // Eliminazione dei valori sottostanti alla diagonale
            for k in i+1..n {
                let fattore = a[k][i] / a[i][i];
                for j in i+1..n {
                    a[k][j] -= fattore * a[i][j];
                }
            }
        }
        //se il numero degli scambi è dispari cambio il segno del determinante
        if scambi % 2 == 1 {
            det = -det;
        }
        //ritorno il determinante
        Ok(det)
    }
}



