// fn contar_posicoes_inteiros(array: &[i32]) -> usize {
//     array.len()
// }

// fn contar_posicoes_floats(array: &[f64]) -> usize {
//     array.len()
// }

// fn contar_posicoes_strings(array: &[&str]) -> usize {
//     array.len()
// }

// fn main() {
//     let array_inteiros: [i32; 5] = [1, 2, 3, 4, 5];
//     let array_floats: [f64; 4] = [1.1, 2.2, 3.3, 4.4];
//     let array_strings: [&str; 3] = ["um", "dois", "três"];

//     println!("Posições no array de inteiros: {}", contar_posicoes_inteiros(&array_inteiros));
//     println!("Posições no array de floats: {}", contar_posicoes_floats(&array_floats));
//     println!("Posições no array de strings: {}", contar_posicoes_strings(&array_strings));
// }

//==== função duplicada por objetivo ====
fn quantidade_digitos_inteiro(i: i32) -> usize {
    i.to_string().chars().count()
}

fn quantidade_digitos_float(f: f64) -> usize {
    f.to_string().chars().count()
}

fn quantidade_caracteres_string(s: &str) -> usize {
    s.chars().count()
}

fn main() {
    let int_val: i32 = 12345;
    let float_val: f64 = 123.45;
    let string_val: &str = "Olá josé";

    println!("Quantidade de dígitos no inteiro: {}", quantidade_digitos_inteiro(int_val));
    println!("Quantidade de dígitos no float: {}", quantidade_digitos_float(float_val));
    println!("Quantidade de caracteres na string: {}", quantidade_caracteres_string(string_val));
}
