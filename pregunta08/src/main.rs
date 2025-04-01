fn lista_enteros(lista:&Vec<i32>) -> Vec<Vec<i32>>{
    let mut grupos: Vec<Vec<i32>> = Vec::new();
    let mut grupo_actual: Vec<i32> = Vec::new();

    for &num in lista {

        if grupo_actual.len() == 0 || num == grupo_actual[grupo_actual.len()-1]+1{
            grupo_actual.push(num);
        }else{
            grupos.push(grupo_actual);
            grupo_actual = vec![num];
        }
    }
    // Agregar el último grupo
    if grupo_actual.len() > 0 {
        grupos.push(grupo_actual);
    }

    grupos

}


fn main() {
    let lista = vec![1,2,3,5,6,9];
    let resultado = lista_enteros(&lista);
    // resultado final
println!("{:?}", resultado);
}