
use std::io::{self, Write};
mod fisica;
use crate::{fisica::Vettore, matematica::{Coseno, Seno, converti_gradi_radianti}};
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
    let f1: Seno = Seno::new( 1.0, 2.0, 0.0);
    print!("{}",f1.derivata().print());
}



