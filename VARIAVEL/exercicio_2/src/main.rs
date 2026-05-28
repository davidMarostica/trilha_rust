use std::io;

fn main() {
    loop {
        println!("==============================");
        println!("        MENU TABUADA          ");
        println!("==============================");
        println!("Digite um número para ver a tabuada");
        println!("Ou digite 0 para sair");
        println!("==============================");

        // Lê entrada do usuário
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).unwrap();

        // Converte para número inteiro
        let valor: i32 = entrada.trim().parse().unwrap_or(-1);

        if valor == 0 {
            println!("Saindo do programa...");
            break;
        }

        if valor < 0 {
            println!("Número inválido, tente novamente!");
            continue;
        }

        println!("Você escolheu o número: {}", valor);
        println!("Tabuada do {}", valor);

        for multiplicador in 1..=10 {
            println!("{} x {} = {}", valor, multiplicador, valor * multiplicador);
        }
    }
}
