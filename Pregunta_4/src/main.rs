use rand::Rng;


//en el ejercicio no pide agregarlo desde consola M*N (tamaño de matriz), en ese caso se solicitaría a usuario.
struct Resultado {
    matriz: Vec<Vec<i16>>,
    promedio: f32,
}

fn generar_matriz_promediar_imprimir()->Resultado{
    let mut matriz: Vec<Vec<i16>> = vec![vec![0; 3]; 2];
    let mut suma: i32 = 0;

    for fila in &mut matriz{
        for valor in fila.iter_mut(){
            *valor = rand::rng().random_range(10..=100);
            print!("{}  ", *valor);
            suma += *valor as i32;
        }
        println!();
    }
    let cantidad_elementos: f32 = (matriz.len()*matriz[0].len()) as f32;
    let promedio: f32 = suma as f32 / cantidad_elementos;

    Resultado {matriz, promedio}
}

fn comparar(resultado: Resultado){
    let matriz = resultado.matriz;
    let promedio = resultado.promedio;
    let mut mayores: u16 = 0;

    for fila in matriz{
        for valor in fila.iter(){
            if promedio < *valor as f32 {
                mayores += 1;
            }
        }
    }
    println!("{} celdas son mayores que el promedio", mayores);
}

/*Función usada para experimentar; en Rust se pueden unar iteradores y &mut T que sería una referencia
(una cajita que tiene un valor dentro)
fn imprimir_matriz(matriz: Vec<Vec<i16>>){
    let mut valor: i16;
    for i in 0..2{
        for j in 0..3{
            valor = matriz[i][j];
            print!("{}  ", valor);
        }
        println!();
    }
}*/

fn main() {
    let resultado = generar_matriz_promediar_imprimir();
    println!("Promedio {}: ", resultado.promedio);
    comparar(resultado);
}
