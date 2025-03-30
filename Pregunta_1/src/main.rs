use std::io;
//Uso en CMD: cargo add rand
use rand::Rng;
use std::collections::HashMap;

fn leer() -> String{
    let mut n: String = String::new();
    println!("Ingresa N: ");
    io::stdin().read_line(&mut n).expect("error");

    n.trim().to_string()
}

fn crear_vector(n: u16)->Vec<u16>{
    let mut vector = Vec::with_capacity(n as usize);
    let mut i: u16 = 0;
    while i < n {
        vector.push(rand::rng().random_range(0..=20));
        i += 1;
    }
    vector
}

fn imprimir_vector(vector: &Vec<u16>){
    println!("Lista Generada: {:?}", vector);
}

fn promedio(vector: &Vec<u16>){
    
    let mut sum: u32 = 0;
    let promedio: f32;
    for i in 0..vector.len(){
        sum = sum + vector[i] as u32;
    }
    promedio = sum as f32 / vector.len() as f32;
    println!("Promedio: {}", promedio);

}

fn mediana(vector: &mut Vec<u16>){
    vector.sort_unstable();
    let longitud = vector.len();

    if longitud%2 == 1 {
        println!("Mediana: {}", vector[longitud/2] as f64);
    }else{
        let mitad_1 = vector[longitud/2 - 1] as f64;
        let mitad_2 = vector[longitud/2] as f64;
        println!("Mediana: {}", (mitad_1 + mitad_2)/2.0);
    }

}

fn moda(vector: &Vec<u16>){
    let mut mapeo_de_frecuencia: HashMap<u16, u32> = HashMap::new();

    for &num in vector{
        *mapeo_de_frecuencia.entry(num).or_insert(0) += 1;
    }

    let moda = mapeo_de_frecuencia.iter()
        .max_by_key(|&(_, frec)| frec)
        .map(|(&num, _)| num);
    match moda {
        Some(value) => println!("Moda: {}", value),
        None => println!("No hay moda (vector vacío)"),
    }
}

fn main() {
    let n:u16 = loop{
        let entrada = leer();
        match entrada.trim().parse::<u16>() {
            Ok(num) if num > 3 => break num,
            Ok(_) => println!("El número es muy pequeño. Coloque un número más grande."),
            Err(_) => println!("Entrada inválida. Intente de nuevo -_-"),
        }
    };

    println!("Loop Terminado 🚀");

    let mut vector: Vec<u16> = crear_vector(n);
    let _longitud: u16 = vector.len() as u16;
    imprimir_vector(&vector);
    promedio(&vector);
    mediana(&mut vector);
    moda(&vector);

    println!("Programa Terminado 😊");

}