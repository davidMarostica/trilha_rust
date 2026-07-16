
/*
Closures são funções anônimas que podem capturar variáveis do escopo onde foram definidas 
para uso posterior. 

Elas são bastante úteis para tarefas como passagem de comportamento como argumento para outras funções,
construção de abstrações de controle e manipulação de dados de coleções.

Características das Closures
Anônimas: Closures geralmente não têm um nome.

Capturam o ambiente: Podem capturar variáveis do contexto onde são definidas, 
seja por valor ou por referência.

Flexíveis: Podem ser armazenadas em variáveis, passadas como argumentos para outras funções, e mais.

Tipagem forte: Assim como outras funções em Rust, as closures são fortemente tipadas, 
mas o compilador de Rust muitas vezes pode inferir seus tipos automaticamente.

*/
struct Pedido {
    nome_cliente: String,
    valor: f32,
    entregue: bool,
}

fn main() {
    let x = 4;

    let exemplo_closure = |parametro| parametro + x;
    println!("exemplo_closure: {}", exemplo_closure(6));

    let igual_a_x = |z| z == x;
    let y = 4;
    println!("O resultado da comparação é: {}", igual_a_x(y));

    // Agora usamos a struct Pedido
    let pedidos = vec![
        Pedido { nome_cliente: String::from("Alice"), valor: 150.0, entregue: true },
        Pedido { nome_cliente: String::from("Bob"), valor: 250.0, entregue: false },
        Pedido { nome_cliente: String::from("Carol"), valor: 100.0, entregue: true },
    ];

    let valor_total: f32 = pedidos
        .iter()
        .filter(|pedido| pedido.entregue) // closure inline
        .map(|pedido| {
            println!("Pedido entregue para: {}", pedido.nome_cliente);
            pedido.valor
        })
        .sum();

    println!("O valor total dos pedidos entregues é: {:.2}", valor_total);
}
