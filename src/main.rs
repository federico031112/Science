
use std::io::{self, Write};
mod fisica;
use crate::fisica::{Vettore,Vettore3D};
mod matematica;

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
    //inizializzazione del primo vettore 2D
    let vet1: Vettore = Vettore::new(1.0, 0.0);
    //inizializzazione del secondo vettore 2D
    let vet2: Vettore = Vettore::new(0.0, 1.0);
    //calcolo del prodotto vettoriale tra i due vettori e creazione del vettore 3D
    let vet3: Vettore3D = vet1.prodotto_vettoriale_tra_vettori(&vet2);
    //stampa delle componenti del vettore 3D
    println!("{} {} {}",vet3.x,vet3.y,vet3.z); 
}



