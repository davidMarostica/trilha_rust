fn contar_posicoes<T>(array: &[T]) -> usize {
    array.len()
}

fn main() {
    let array_inteiros: [i32; 5] = [1, 2, 3, 4, 5];
    let array_floats: [f64; 4] = [1.1, 2.2, 3.3, 4.4];
    let array_strings: [&str; 3] = ["um", "dois", "três"];

    println!("Posições no array de inteiros: {}", contar_posicoes(&array_inteiros));
    println!("Posições no array de floats: {}", contar_posicoes(&array_floats));
    println!("Posições no array de strings: {}", contar_posicoes(&array_strings));
}
