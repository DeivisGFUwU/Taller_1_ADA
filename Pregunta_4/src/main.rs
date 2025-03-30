use rand::Rng;

fn generar_matriz()->Vec<Vec<i16>>{
    let mut matriz: Vec<Vec<i16>> = vec![vec![0; 3]; 2];

    for i in 0..2{
        for j in 0..3{
            matriz[i][j] = rand::rng().random_range(10..=100);
        }
    }
    matriz
}

fn imprimir_matriz(matriz: Vec<Vec<i16>>){
    let mut valor: i16;
    for i in 0..2{
        for j in 0..3{
            valor = matriz[i][j];
            print!("{}  ", valor);
        }
        println!();
    }
}

fn main() {
    let matriz:Vec<Vec<i16>> = generar_matriz();
    imprimir_matriz(matriz);
    
}
