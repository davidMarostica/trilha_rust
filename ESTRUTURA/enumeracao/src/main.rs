// enum Tipo {
//     Juridica,
//     Fisica,
// }

// struct Pessoa {
//     nome: String,
//     documento: String,
//     tipo: Tipo,
// }

// fn main() {
//     let david: Pessoa = Pessoa {
//         nome: String::from("david"),
//         documento: String::from("12.456.789/9999-00"),
//         tipo: Tipo::Juridica,
//     };

//     let maria: Pessoa = Pessoa {
//         nome: String::from("Maria"),
//         documento: String::from("123.456.789-00"),
//         tipo: Tipo::Fisica,
//     };

//     match david.tipo {
//         Tipo::Juridica => println!("Pessoa jurídica: {} - Documento: {}", david.nome, david.documento),
//         Tipo::Fisica => println!("Pessoa física: {} - Documento: {}", david.nome, david.documento),
//     }

//     match maria.tipo {
//         Tipo::Juridica => println!("Pessoa jurídica: {} - Documento: {}", maria.nome, maria.documento),
//         Tipo::Fisica => println!("Pessoa física: {} - Documento: {}", maria.nome, maria.documento),
//     };
// }
// Exemplo 1: Status de Pedido
// Representa os estados possíveis de um pedido em um sistema de e-commerce.


// Exemplo 1: Status de Pedido



// =======================================================
// 1. ENUNCIADO: Definir estados possíveis
// -------------------------------------------------------
// Este enum representa os estados que um pedido pode assumir
// em um sistema de e-commerce. Cada variante é um estado válido.
enum StatusPedido {
    Pendente,
    Processando,
    Enviado,
    Entregue,
    Cancelado,
}

// =======================================================
// 2. ENUNCIADO: Organizar categorias
// -------------------------------------------------------
// Este enum organiza os dias da semana em categorias fixas.
// Evita o uso de strings soltas como "Segunda".
enum DiaSemana {
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
    Domingo,
}

// =======================================================
// 3. ENUNCIADO: Associar dados
// -------------------------------------------------------
// Este enum mostra como cada variante pode carregar dados.
// Sucesso guarda uma mensagem positiva, Erro guarda uma mensagem de falha.
enum Resultado {
    Sucesso(String),
    Erro(String),
}

// =======================================================
// 4. ENUNCIADO: Facilitar pattern matching
// -------------------------------------------------------
// Este enum define níveis de acesso em um sistema.
// O match garante que todos os casos sejam tratados.
enum NivelAcesso {
    Admin,
    Usuario,
    Convidado,
}

// =======================================================
// 5. ENUNCIADO: Substituir múltiplos structs
// -------------------------------------------------------
// Este enum representa endereços IP.
// Em vez de criar dois structs diferentes (IPv4 e IPv6),
// usamos um único enum com variantes distintas.
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    // ===================================================
    println!("\n=== 1. Definir estados possíveis ===");
    let pedidos = [
        StatusPedido::Pendente,
        StatusPedido::Processando,
        StatusPedido::Enviado,
        StatusPedido::Entregue,
        StatusPedido::Cancelado,
    ];
    for pedido in pedidos {
        match pedido {
            StatusPedido::Pendente => println!("Pedido pendente."),
            StatusPedido::Processando => println!("Pedido em processamento."),
            StatusPedido::Enviado => println!("Pedido enviado."),
            StatusPedido::Entregue => println!("Pedido entregue."),
            StatusPedido::Cancelado => println!("Pedido cancelado."),
        }
    }

    // ===================================================
    println!("\n=== 2. Organizar categorias ===");
    let semana = [
        DiaSemana::Segunda,
        DiaSemana::Terca,
        DiaSemana::Quarta,
        DiaSemana::Quinta,
        DiaSemana::Sexta,
        DiaSemana::Sabado,
        DiaSemana::Domingo,
    ];
    for dia in semana {
        match dia {
            DiaSemana::Segunda => println!("Segunda-feira."),
            DiaSemana::Terca => println!("Terça-feira."),
            DiaSemana::Quarta => println!("Quarta-feira."),
            DiaSemana::Quinta => println!("Quinta-feira."),
            DiaSemana::Sexta => println!("Sexta-feira."),
            DiaSemana::Sabado => println!("Sábado."),
            DiaSemana::Domingo => println!("Domingo."),
        }
    }

    // ===================================================
    println!("\n=== 3. Associar dados ===");
    let resultados = [
        Resultado::Sucesso(String::from("Operação concluída.")),
        Resultado::Erro(String::from("Falha na operação.")),
    ];
    for r in resultados {
        match r {
            Resultado::Sucesso(msg) => println!("Sucesso: {}", msg),
            Resultado::Erro(msg) => println!("Erro: {}", msg),
        }
    }

    // ===================================================
    println!("\n=== 4. Facilitar pattern matching ===");
    let acessos = [
        NivelAcesso::Admin,
        NivelAcesso::Usuario,
        NivelAcesso::Convidado,
    ];
    for acesso in acessos {
        match acesso {
            NivelAcesso::Admin => println!("Acesso total."),
            NivelAcesso::Usuario => println!("Acesso limitado."),
            NivelAcesso::Convidado => println!("Acesso restrito."),
        }
    }

    // ===================================================
    println!("\n=== 5. Substituir múltiplos structs ===");
    let ips = [
        IpAddr::V4(String::from("127.0.0.1")),
        IpAddr::V6(String::from("::1")),
    ];
    for ip in ips {
        match ip {
            IpAddr::V4(addr) => println!("IPv4: {}", addr),
            IpAddr::V6(addr) => println!("IPv6: {}", addr),
        }
    }
}
