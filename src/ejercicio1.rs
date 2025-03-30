use std::{collections::{btree_map::Range, HashMap}, io};
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

println!("Este es el vector {:?}",lista);

let promedio:f64 = (lista.iter().sum::<i32>() as f64)/  (lista.len() as f64);

println!("ESTE ES EL PROMEDIO:{}: ",promedio);

let media= if lista.len()/2 ==0{
    (lista[lista.len()/2 -1 ] + lista[lista.len()/2]) as f64 /2.0
} else {
    lista[lista.len()/2 ] as f64
};
println!("Este es la media {}: ",media);

let media = lista.iter().fold(HashMap::new() , |mut acc ,&num|{
    *acc.entry(num).or_insert(0)+=1;
    acc
}).iter().max_by_key(|&(_,&count)|count).map(|(&num, _)| num);

println!("Este es la media {:?}", media);





}
