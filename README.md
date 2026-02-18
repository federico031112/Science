per il calcolo della matrice bisogna prima creare un vettore di vettori quindi di tipo Vec<Vec<f64>>
in seguito creare ogni riga della matrice creata sopra
ed infine creare la struct per eseguire le operazioni il codice seguente mostra come stampare e verificare la corretta creazione della matrice:

-----------------------------------------------------
-----------------------------------------------------
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
-----------------------------------------------------
-----------------------------------------------------
