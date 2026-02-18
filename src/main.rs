
use std::io::{self, Write};
mod fisica;
use crate::{fisica::Vettore, matematica::{Coseno, Seno, converti_gradi_radianti, Matrice}};
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
    let mut matrice: Vec<Vec<f64>> = Vec::new();
    let riga1 = vec![1.0,2.0,3.0];
    let riga2 = vec![5.0,9.0,8.0];
    let riga3 = vec![7.0,2.0,6.0];
    matrice.push(riga1);
    matrice.push(riga2);
    matrice.push(riga3);
    if let Some(matrix) = Matrice::new(3,3,matrice){
        for i in 0..matrix.righe{
            for n in 0..matrix.colonne{
                print!("{}",matrix.data[i][n]);
            }
            println!("")
        }
    }else{
        println!("errore");
    }
    
    
}



