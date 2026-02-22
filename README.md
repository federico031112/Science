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

esempio di utilizzo di una forma geometrica con la libreria geometria, il processo è molto intuitivo e si può applicarea a qualsiasi forma

---------------------------------------------------------------------------
---------------------------------------------------------------------------
```rust
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
```
-----------------------------------------------------------------------------
-----------------------------------------------------------------------------

esempio di codice per il calcolo del determinante di una matrice

-----------------------------------------------------------------------------
-----------------------------------------------------------------------------
```rust
fn main() -> Result<(), String>{
    let mut matrice: Vec<Vec<f64>> = Vec::new();
    let riga1 = vec![1.0,2.0,3.0];
    let riga2 = vec![5.0,9.0,8.0];
    let riga3 = vec![7.0,2.0,6.0];
    matrice.push(riga1);
    matrice.push(riga2);
    matrice.push(riga3);
    let matrix = Matrice::new(3,3,matrice)?;
        for i in 0..matrix.righe{
            for n in 0..matrix.colonne{
                print!("{}",matrix.data[i][n]);
            }
            println!("")
        }
    println!("determinante = {}",matrix.calcola_determinante()?);

    Ok(())
}
```
------------------------------------------------------------------------------
------------------------------------------------------------------------------

test del modulo di fisica riguardo ai vettori 3D

------------------------------------------------------------------------------
------------------------------------------------------------------------------
```rust
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
```
------------------------------------------------------------------------------
------------------------------------------------------------------------------

test del modulo potenza con l'impiego dei numeri immaginari tramite Complex per maggiore precisione e velocità

------------------------------------------------------------------------------
------------------------------------------------------------------------------
```rust
fn main() -> Result<(), String>{
    let f1 = Potenza::new(1.0,3.5);
    let risultato = f1.calcola(-2.0);
    match risultato {
        Numero::Reale(x) => println!("risultato = {}",x),
        Numero::Complesso(c) => println!("risultato = {}",c)
    }
    Ok(())
}
```
------------------------------------------------------------------------------
------------------------------------------------------------------------------