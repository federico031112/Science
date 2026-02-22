
use std::io::{self, Write};

use crate::{fisica::{Vettore3D, calcolo_componenti_vettore3D}, matematica::{Potenza, Numero} };
mod matematica;
mod fisica;
mod geometria;

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
fn main() -> Result<(), String>{
    let f1 = Potenza::new(1.0,3.5);
    let risultato = f1.calcola(-2.0);
    match risultato {
        Numero::Reale(x) => println!("risultato = {}",x),
        Numero::Complesso(c) => println!("risultato = {}",c)
    }
    Ok(())
}



