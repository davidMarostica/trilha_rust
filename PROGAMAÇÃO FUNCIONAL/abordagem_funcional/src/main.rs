struct Pedido {
    nome_cliente: String,
    valor: f32,
    entregue: bool,
}

/*
Em resumo:
- `impl Fn` é eficiente quando sabemos exatamente o tipo da closure que será retornada.
- `Box<dyn Fn>` dá flexibilidade para retornar diferentes closures que implementam o mesmo trait.
*/

// fn soma(x: i32, y: i32) -> impl Fn(i32) -> Box<dyn Fn(i32) -> i32> {
//     let r: i32 = x + y;
//     move |multiplicador| {
//         let r_mult = r * multiplicador;
//         Box::new(move |sub| r_mult - sub)
//     }
// }

// fn main() {
//     // Exemplo com closures aninhadas
//     let multiplica = soma(5, 3); // 5 + 3 = 8
//     let subtrai = multiplica(2); // 8 * 2 = 16
//     let resultado = subtrai(4);  // 16 - 4 = 12
//     println!("O resultado é: {}", resultado); // O resultado é: 12

//     // Usando a struct Pedido para evitar warning
//     let pedidos = vec![
//         Pedido { nome_cliente: String::from("Alice"), valor: 150.0, entregue: true },
//         Pedido { nome_cliente: String::from("Bob"), valor: 250.0, entregue: false },
//     ];

//     let total: f32 = pedidos
//         .iter()
//         .filter(|p| p.entregue)
//         .map(|p| {
//             println!("Pedido entregue para: {}", p.nome_cliente);
//             p.valor
//         })
//         .sum();

//     println!("Valor total dos pedidos entregues: {:.2}", total);
// }

fn cria_somador(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

fn main() {
        // Usando a struct Pedido para evitar warning
    let pedidos = vec![
        Pedido { nome_cliente: String::from("Alice"), valor: 150.0, entregue: true },
        Pedido { nome_cliente: String::from("Bob"), valor: 250.0, entregue: false },
    ];

    let total: f32 = pedidos
        .iter()
        .filter(|p| p.entregue)
        .map(|p| {
            println!("Pedido entregue para: {}", p.nome_cliente);
            p.valor
        })
        .sum();

    let somador_10 = cria_somador(10);
    println!("10 + 5 = {}", somador_10(5));
    println!("10 + 10 = {}", somador_10(10));
    println!("10 + 30 = {}", somador_10(30));
    println!("10 + 20 = {}", somador_10(20));
    println!("Valor total dos pedidos entregues: {:.2}", total);
}