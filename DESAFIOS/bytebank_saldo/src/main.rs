use std::io;

fn main() {
    // Lê a linha de entrada do usuário (saldo e valor da compra)
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro na leitura");

    // Converte a entrada em dois inteiros positivos
    let valores: Vec<u32> = entrada
        .split_whitespace()
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let saldo = valores[0];
    let compra = valores[1];

    // Verifique se o saldo é suficiente e imprima a mensagem correta
    if saldo >= compra {
        println!("Compra aprovada");
    } else {
        println!("Saldo insuficiente");
    }
}
