////// ===== Operador Option ======
fn encontrar_divisor(numero: i32) -> Option<i32> {
    if numero % 2 == 0 {
        Some(2) // Um divisor encontrado
    } else if numero % 3 == 0 {
        Some(3) // Outro divisor encontrado
    } else {
        None // Nenhum divisor encontrado
    }
}

fn main() {
    let numero = 1;
    match encontrar_divisor(numero) {
        Some(divisor) => println!("Divisor encontrado: {}", divisor),
        None => println!("Nenhum divisor encontrado para {}", numero),
    }
}