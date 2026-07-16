struct Pedido {
    nome_cliente: String,
    valor: f32,
    entregue: bool,
}

fn main() {
    let pedidos = vec![
        Pedido { nome_cliente: String::from("Alice"), valor: 150.0, entregue: true },
        Pedido { nome_cliente: String::from("Bob"), valor: 250.0, entregue: false },
        Pedido { nome_cliente: String::from("Carol"), valor: 100.0, entregue: true },
    ];

    let valor_total: f32 = pedidos
        .iter()
        .filter(|p| p.entregue)
        .map(|p| {
            println!("Pedido entregue para: {}", p.nome_cliente); // usa o campo
            p.valor
        })
        .sum();

    println!("O valor total dos pedidos entregues é: {:.2}", valor_total);
}
