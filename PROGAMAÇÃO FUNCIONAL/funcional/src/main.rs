struct Pedido {
    nome_cliente: String,
    valor: f32,
    entregue: bool,
}

// Função que retorna função (closure)
fn pedido_entregue(p: &Pedido) -> bool {
    p.entregue
}

fn main() {
    let pedidos = vec![
        Pedido { nome_cliente: String::from("Alice"), valor: 150.0, entregue: true },
        Pedido { nome_cliente: String::from("Bob"), valor: 250.0, entregue: false },
        Pedido { nome_cliente: String::from("Carol"), valor: 100.0, entregue: true },
    ];
        //// === Conceito de função que retorna função ===
    // iter() - Cria um iterador sobre a coleção.
    // filter(|pedido| pedido.entregue) - Filtra os clientes entregues
    // map(|pedido| pedido.valor) - mapeia os itens retornando um array de f32
    // sum() - Soma os dados retornados pelo array

    let valor_total: f32 = pedidos
        .iter()
        .filter(|&pedido| pedido_entregue(pedido))
        .map(|pedido| {
            // Usa o campo nome_cliente para evitar warning
            println!("Pedido entregue para: {}", pedido.nome_cliente);
            pedido.valor
        })
        .sum();

    println!("O valor total dos pedidos entregues é: {:.2}", valor_total);
}
