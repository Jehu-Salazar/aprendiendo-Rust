use std::{io, thread::JoinHandle};
use rand::{rngs::OsRng, Rng};



use rand::rngs::{self, ThreadRng};

pub fn resolver(){
    println!("Esta es la resolucion de el ejercccio numero 2");

    /*
    Problema 2: Diagonal Secundaria (2 puntos).
    Crea una funci´on que genere una matriz cuadrada N ×N de elementos aleatorios y que retorne:
    La matriz N ×N generada.
    Los elementos de la diagonal secundaria (de derecha a izquierda, desde arriba).
    La suma de los elementos de dicha diagonal.
     */

    let mut entrada = String::new();
    let mut matriz: Vec<Vec<i32>> = Vec::new();
// lectur de datos
    println!("Ingrese el numero NxN");
    io::stdin().read_line(&mut entrada).expect("Error de entrada de NxN");
    let numero : usize = entrada.trim().parse().expect("Eror al convertir String");

    println!("este es N {}", numero);
    let mut rng:ThreadRng = rand::thread_rng();


    for i in 0..numero{
        let mut fila: Vec<i32> = Vec::new();

        for j in 0..numero {

            let aleatorio = rng.gen_range(0..=20);
            fila.push(aleatorio);

            
        }
        matriz.push(fila);
    }

    println!("ESTA ES LA MATRIZ {:?}",matriz);

    println!("imprimiendo la diagonal secundaria");
    let filas= matriz.len();
    let columnas = if filas>0 { matriz[0].len()} else{0};

    let mut diagonal_secundaria: Vec<i32> = Vec::new();

    if columnas==filas {
        for i in 0..filas  {
            println!("Numero {}",matriz[i][columnas-1-i]);
            diagonal_secundaria.push(matriz[i][columnas-1-i]);
            
        }
        println!("diagonal secundaria {:?}",diagonal_secundaria);
    } else {
        println!("No hay diagonal ");
    }

    let mut suma =0;

    for i in 0..diagonal_secundaria.len() {
        suma = suma + diagonal_secundaria[i];
        
    }

    println!("La suma de la diagonal secundaria es : {}",suma);
    

}