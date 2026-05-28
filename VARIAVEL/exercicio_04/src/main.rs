use std::io;

fn main() {
    loop {
        println!("===== MENU =====");
        println!("1 - Soma");
        println!("2 - Subtração");
        println!("3 - Multiplicação");
        println!("4 - Divisão");
        println!("5 - Tabuada");
        println!("6 - Sair");
        println!("Escolha uma opção:");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro ao ler opção");
        let opcao: i32 = opcao.trim().parse().unwrap_or(0);

        if opcao == 6 {
            println!("Saindo...");
            break;
        }

        if opcao == 5 {
            println!("Digite o número para gerar a tabuada:");
            let mut n = String::new();
            io::stdin().read_line(&mut n).expect("Erro ao ler número");
            let n: i32 = n.trim().parse().unwrap_or(0);
            tabuada(n);
            continue;
        }

        println!("Digite o primeiro número:");
        let mut n1 = String::new();
        io::stdin().read_line(&mut n1).expect("Erro ao ler número");
        let n1: f64 = n1.trim().parse().unwrap_or(0.0);

        println!("Digite o segundo número:");
        let mut n2 = String::new();
        io::stdin().read_line(&mut n2).expect("Erro ao ler número");
        let n2: f64 = n2.trim().parse().unwrap_or(0.0);

        match opcao {
            1 => println!("Resultado da soma: {}", soma(n1, n2)),
            2 => println!("Resultado da subtração: {}", subtracao(n1, n2)),
            3 => println!("Resultado da multiplicação: {}", multiplicacao(n1, n2)),
            4 => {
                if n2 != 0.0 {
                    println!("Resultado da divisão: {}", divisao(n1, n2));
                } else {
                    println!("Erro: divisão por zero!");
                }
            }
            _ => println!("Opção inválida!"),
        }
    }
}

fn soma(x: f64, y: f64) -> f64 {
    x + y
}

fn subtracao(x: f64, y: f64) -> f64 {
    x - y
}

fn multiplicacao(x: f64, y: f64) -> f64 {
    x * y
}

fn divisao(x: f64, y: f64) -> f64 {
    x / y
}

fn tabuada(n: i32) {
    println!("Tabuada do {}", n);
    for i in 1..=10 {
        println!("{} x {} = {}", n, i, n * i);
    }
}
