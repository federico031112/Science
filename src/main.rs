
use std::io::{self, Write};

use crate::fisica::{Vettore3D, calcolo_componenti_vettore3D};
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
    let vec1: Vettore3D = Vettore3D::new(1.0, 1.0, 1.0);
    let vec2: Vettore3D = Vettore3D::new(1.0, 1.0, 1.0);
    println!("modulo vec1 = {}",vec1.calcola_modulo());
    let versore1 = vec1.calcola_versore()?;
    println!("versore: x = {} y = {} z = {}",versore1.x,versore1.y,versore1.z);
    let somma = vec1.somma_vettori_3D(vec2);
    println!("somma: x = {} y = {} z = {}",somma.x,somma.y,somma.z);
    let vec3 = calcolo_componenti_vettore3D(vec1.calcola_modulo(), versore1.x.acos(), versore1.y.acos(), versore1.z.acos());
    println!("vettore 3: x = {} y = {} z = {}",vec3.x,vec3.y,vec3.z);
    Ok(())
}



