struct Pedido {
    nome_cliente: String,
    valor: f32,
    entregue: bool,
}

fn soma(x: i32, y: i32) -> impl Fn(i32) -> i32 {
    let r = x + y;
    move |multiplicador| r * multiplicador // closure
}

fn main() {
    let resultado_soma = soma(5, 3); // 5 + 3 = 8
    let resultado_final = resultado_soma(2); // 8 * 2 = 16

    println!("Resultado da multiplicação: {}", resultado_final);

    // Usando a struct Pedido para evitar warning
    let pedidos = vec![
        Pedido { nome_cliente: String::from("Alice"), valor: 150.0, entregue: true },
        Pedido { nome_cliente: String::from("Bob"), valor: 250.0, entregue: false },
    ];

    // Closure inline para somar apenas pedidos entregues
    let total: f32 = pedidos
        .iter()
        .filter(|p| p.entregue)
        .map(|p| {
            println!("Pedido entregue para: {}", p.nome_cliente);
            p.valor
        })
        .sum();

    println!("Valor total dos pedidos entregues: {:.2}", total);
}
