use std::io;

fn main() {
    // Lê uma linha da entrada padrão
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();

        // Se houver exatamente dois elementos, imprime a mensagem personalizada.
        if parts.len() == 2 {
            let nome = parts[0];
            let conta = parts[1];
            println!("Welcome, {}! Your account type is {}.", nome, conta);
        } else {
            println!("Invalid input.");
        }
    }
}
