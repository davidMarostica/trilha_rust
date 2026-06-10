// Exemplo de Pattern Matching (Combinação de Padrões em Rust)

enum Comando {
    Iniciar(String),
    Parar,
    Pausar,
    Continuar,
}

fn processar_comando(comando: Comando) {
    match comando {
        Comando::Iniciar(mensagem) => println!("Iniciando com: {}", mensagem),
        Comando::Parar => println!("Parando..."),
        Comando::Pausar => println!("Pausando..."),
        Comando::Continuar => println!("Continuando..."),
    }
}

fn main() {
    let comando1 = Comando::Iniciar(String::from("Iniciar aplicação"));
    let comando2 = Comando::Parar;
    let comando3 = Comando::Pausar;
    let comando4 = Comando::Continuar;

    processar_comando(comando1);
    processar_comando(comando2);
    processar_comando(comando3);
    processar_comando(comando4);
}
