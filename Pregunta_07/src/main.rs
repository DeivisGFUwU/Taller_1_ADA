fn es_valido (matriz: [[u8; 9]; 9]) -> bool {
    for fila in 0..9 {
        let mut seen = [false; 10];
        for columna in 0..9 {
            let num = matriz[fila][columna];
            if num != 0 {
                if seen[num as usize]{
                    return false;
                }
                seen[num as usize] = true;
            }
        }
    }

    for columna in 0..9 {
        let mut seen = [false; 10];
        for fila in 0..9 {
            let num = matriz[fila][columna];
            if num != 0 {
                if seen[num as usize]{
                    return false;
                }
                seen[num as usize] = true;
            }
        }
    }

    for inicio_fila in (0..9).step_by(3) {
        for inicio_col in (0..9).step_by(3) {
            let mut seen = [false; 10];
            for i in 0..3 {
                for j in 0..3 {
                    let num = tablero[inicio_fila + i][inicio_col + j];
                    if num != 0 {
                        if seen[num as usize] {
                            return false;
                        }
                        seen[num as usize] = true;
                    }
                }
            }
        }
    }

    true
}

fn main() {
    let matriz: [[u8; 9]; 9] = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    
    if es_valido(matriz){
        println!("El tablero es válido.");
    }else{
        println!("El tablero no es válido.")
    }
    

}
