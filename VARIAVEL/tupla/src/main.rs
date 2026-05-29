// fn main() {
//     // Criando uma tupla com tipos diferentes
//     let tupla: (i32, f64, &str) = (42, 3.14, "Olá, Rust!");

//     // Acessando elementos da tupla
//     println!("Primeiro elemento: {}", tupla.0);
//     println!("Segundo elemento: {}", tupla.1);
//     println!("Terceiro elemento: {}", tupla.2);
// }

// use std::collections::HashMap;

// fn main() {
//     let mut mapa: HashMap<(i32, i32), &str> = HashMap::new();

//     mapa.insert((0, 0), "Origem");
//     mapa.insert((10, 5), "Ponto A");
//     mapa.insert((20, 15), "Ponto B");

//     println!("Descrição do ponto (10,5): {}", mapa.get(&(10, 5)).unwrap());
// }


// fn estatisticas(valores: &[i32]) -> (i32, i32, f64) {
//     let soma: i32 = valores.iter().sum();
//     let minimo: i32 = *valores.iter().min().unwrap();
//     let media: f64 = soma as f64 / valores.len() as f64;

//     (soma, minimo, media)
// }

// fn main() {
//     let numeros = vec![10, 20, 30, 40];
//     let (soma, minimo, media) = estatisticas(&numeros);

//     println!("Soma: {}", soma);
//     println!("Mínimo: {}", minimo);
//     println!("Média: {:.2}", media);
// }

// tupla dentro de uma tupla

fn main() {
    // Tupla que representa um usuário com dados agrupados
    let usuario: ((i32, &str), (bool, f64)) = ((101, "David"), (true, 72.5));

    // Acessando elementos diretamente
    println!("ID: {}", usuario.0.0);
    println!("Nome: {}", usuario.0.1);
    println!("Ativo: {}", usuario.1.0);
    println!("Peso: {}", usuario.1.1);
}
