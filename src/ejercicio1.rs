use std::{collections::btree_map::Range, io};
use rand::{rngs::OsRng, Rng};


pub fn resolver() {
    println!("Ejercicio 1 resuelto");

    /* Crea una funci´on que genere una lista aleatoria de N calificaciones con valores entre 0 y 20, y
 calcule el promedio, la mediana y la moda. Si hay menos de 3 notas, retorna None. */

let mut lista: Vec<i32> =Vec::new();
let mut entrada = String::new();

println!("Ingresa el numero N");
io::stdin().read_line(&mut entrada).expect("error al leer");

let numero:i32 =entrada.trim().parse().expect("Ingrese un numero ");

println!("El numero ingresado es {}",numero);
let mut  rng= rand::thread_rng();

for i in 0..numero {

    
    let aleatorio=rng.gen_range(0..=20);
    lista.push(aleatorio);

    
}

println!("{:?}",lista);



}
