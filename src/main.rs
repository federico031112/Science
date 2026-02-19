
use std::io::{self, Write};
mod fisica;
use crate::fisica::{Vettore,Vettore3D};
mod matematica;
mod geometria;
use crate::geometria::Quadrato;

fn leggi_64(messaggio: &str) -> f64{
    loop{
        print!("{}", messaggio);
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("errore");

        match buffer.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("\x1b[31mErrore inserisci un numero valido\x1b[0m")
        }
    }
}
fn main() {
    //inizializzo un quadrato: i valori sconosciuti li inizializzo con 0.0
    let mut q = Quadrato::new(1.0, 0.0, 0.0);
    //calcolo dell'area del quadrato
    q.calcola_area();
    //calcolo della diagonale del quadrato
    q.calcola_diagonale();
    //stampa dei risultati
    println!("area: {}m2 diagonale: {}m",q.area,q.diagonale);
    //inizializzazione di un secondo quadrato conoscendo la sua area
    let mut q2 = Quadrato::new(0.0, 0.0, 1.0);
    //calcolo del lato
    q2.calcola_lato_da_area();
    //calcolo della diagonale
    q2.calcola_diagonale();
    //stampa dei risultati
    println!("lato: {} diagonale: {}",q2.lato,q2.diagonale);
}



