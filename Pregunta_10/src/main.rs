fn union (lista1: Vec<u32>, lista2: Vec<u32>) -> Vec<u32>{
    let ( mayor, menor): (Vec<u32>, Vec<u32>) = if lista1.len() >= lista2.len() {
        (lista1, lista2)
    } else {
        (lista2, lista1)
    };

    let mut resultado: Vec<u32> = Vec::with_capacity(mayor.len() + menor.len());

    let mut i_mayor = 0;
    let mut i_menor = 0;

    for i in 0..(mayor.len() + menor.len()) {
        if i % 2 == 0 {
            if i_mayor < mayor.len() {
                resultado.push(mayor[i_mayor]);
                i_mayor += 1;
            }
        } else {
            if i_menor < menor.len() {
                resultado.push(menor[i_menor]);
                i_menor += 1;
            } else if i_mayor < mayor.len() {
                resultado.push(mayor[i_mayor]);
                i_mayor += 1;
            }
        }
    }

    resultado
}
fn main() {
    let lista1: Vec<u32>= vec![1, 2, 3, 4, 5, 6];
    let lista2: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", union(lista1, lista2));
}
