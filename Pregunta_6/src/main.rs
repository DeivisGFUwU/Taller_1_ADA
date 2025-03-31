fn indices (lista: &[i16], objetivo: i16){
    let mut indices: Vec<i16> = Vec::new();
    
    for i in 0..lista.len(){
        if objetivo == lista[i] {
            indices.push(i as i16);
        }
    }
    let inicial: i16 = *indices.first().unwrap_or(&-1);
    let ultimo:i16 = *indices.last().unwrap_or(&-1);
    println!("({}, {})", inicial, ultimo);
}

fn main() {
    let lista: [i16; 7] = [1, 2, 2, 2, 3, 4, 5];
    let objetivo: i16 = 2;

    indices(&lista, objetivo);
}
