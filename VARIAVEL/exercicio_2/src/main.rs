use std::io;

fn main() {
    println!("Digite o número da tabuada que deseja ver:");

    // Lê entrada do usuário
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();

    // Converte para número inteiro
    let valor_tabuada: i32 = entrada.trim().parse().unwrap_or(1);

    println!("Tabuada do {}", valor_tabuada);

    // Loop de 1 a 10
    for multiplicador in 1..=10 {
        println!("{} x {} = {}", valor_tabuada, multiplicador, valor_tabuada * multiplicador);
    }
}
