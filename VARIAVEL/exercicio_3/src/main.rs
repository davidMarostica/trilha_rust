use std::io;

fn main() {
    let mut continuar = true;

    while continuar {
        println!("==============================");
        println!("        MENU TABUADA          ");
        println!("==============================");
        println!("Digite um número para ver a tabuada");
        println!("(Digite 0 para sair)");
        println!("Opção 1 - Tabuada até 10");
        println!("Opção 2 - Tabuada até 20");
        println!("Opção 3 - Tabuada invertida");
        println!("==============================");

        // Lê número
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).unwrap();
        let valor: i32 = match entrada.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida, tente novamente!");
                continue;
            }
        };

        if valor == 0 {
            println!("Saindo do programa...");
            continuar = false;
            continue;
        }

        // Lê opção
        println!("Escolha a opção (1, 2 ou 3):");
        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).unwrap();
        let opcao: i32 = match opcao.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Opção inválida!");
                continue;
            }
        };

        println!("Você escolheu o número: {}", valor);

        match opcao {
            1 => {
                println!("Tabuada até 10:");
                let mut multiplicador = 1;
                while multiplicador <= 10 {
                    println!("{} x {} = {}", valor, multiplicador, valor * multiplicador);
                    multiplicador += 1;
                }
            }
            2 => {
                println!("Tabuada até 20:");
                let mut multiplicador = 1;
                while multiplicador <= 20 {
                    println!("{} x {} = {}", valor, multiplicador, valor * multiplicador);
                    multiplicador += 1;
                }
            }
            3 => {
                println!("Tabuada invertida (10 até 1):");
                let mut multiplicador = 10;
                while multiplicador >= 1 {
                    println!("{} x {} = {}", valor, multiplicador, valor * multiplicador);
                    multiplicador -= 1;
                }
            }
            _ => println!("Opção inválida!"),
        }
    }
}
