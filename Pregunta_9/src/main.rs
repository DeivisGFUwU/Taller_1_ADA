use std::io;

fn es_triangular_superior(matriz: &Vec<Vec<i32>>) -> bool {
    let n = matriz.len();
    for i in 1..n {
        for j in 0..i {
            if matriz[i][j] != 0 {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut entrada = String::new();
    println!("Ingrese el tamaño de la matriz n:");
    io::stdin().read_line(&mut entrada).expect("Error al leer");
    let n: usize = entrada.trim().parse().expect("Ingrese un número válido");

    let mut matriz = Vec::new();
    println!("Ingrese los elementos de la matriz fila por fila:");
    
    for _ in 0..n {
        let mut fila = String::new();
        io::stdin().read_line(&mut fila).expect("Error al leer");
        let valores: Vec<i32> = fila
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Ingrese solo números válidos"))
            .collect();
        
        if valores.len() != n {
            panic!("Cada fila debe tener exactamente {} elementos", n);
        }
        
        matriz.push(valores);
    }
    
    if es_triangular_superior(&matriz) {
        println!("La matriz es triangular superior.");
    } else {
        println!("La matriz NO es triangular superior.");
    }
}