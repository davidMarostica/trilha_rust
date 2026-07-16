// struct Pedido {
//     nome_cliente: String,
//     valor: f32,
//     entregue: bool,
// }

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

// fn cria_somador(x: i32) -> Box<dyn Fn(i32) -> i32> {
//     Box::new(move |y| x + y)
// }

// fn main() {
//         // Usando a struct Pedido para evitar warning
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

//     let somador_10 = cria_somador(10);
//     println!("10 + 5 = {}", somador_10(5));
//     println!("10 + 10 = {}", somador_10(10));
//     println!("10 + 30 = {}", somador_10(30));
//     println!("10 + 20 = {}", somador_10(20));
//     println!("Valor total dos pedidos entregues: {:.2}", total);
// }

// === Abordagem funcional calculo de salário =======

fn main() {
    let salario_bruto = 5000.0;

    // Cálculo do salário líquido
    fn calcular_salario_liquido(salario_bruto: f64) -> f64 {
        // Função interna para desconto do plano de saúde
        fn desconto_plano_saude(salario: f64) -> f64 {
            salario * 0.10 // Desconto de 10%
        }

        // Função interna para desconto do plano dentário
        fn desconto_plano_dentario(salario: f64) -> f64 {
            salario * 0.05 // Desconto de 5%
        }

        // Função interna para desconto de vale-refeição
        fn desconto_vale_refeicao(salario: f64) -> f64 {
            salario * 0.03 // Desconto de 3%
        }

        let desconto_saude = desconto_plano_saude(salario_bruto);
        let desconto_dentario = desconto_plano_dentario(salario_bruto);
        let desconto_refeicao = desconto_vale_refeicao(salario_bruto);

        salario_bruto - desconto_saude - desconto_dentario - desconto_refeicao
    }

    let salario_liquido = calcular_salario_liquido(salario_bruto);
    println!("Salário líquido: {:.2}", salario_liquido);
}


