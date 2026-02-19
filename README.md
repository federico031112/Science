per il calcolo della matrice bisogna prima creare un vettore di vettori quindi di tipo Vec<Vec<f64>>
in seguito creare ogni riga della matrice creata sopra
ed infine creare la struct per eseguire le operazioni il codice seguente mostra come stampare e verificare la corretta creazione della matrice:

-----------------------------------------------------
-----------------------------------------------------
```rust
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
```
-----------------------------------------------------
-----------------------------------------------------

per calcolare il prodotto vettoriale tra due vettori bidimesionali bisogna prima inizializzare due vettori bidimensionali
in seguito occorre inizializzare un vettore tridimesionale attribuendogli come valore il prodotto vettoriale tra i primi due
ecco il codice di esempio:

-------------------------------------------------------------------------------------
-------------------------------------------------------------------------------------
```rust
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
```
-------------------------------------------------------------------------------------
-------------------------------------------------------------------------------------