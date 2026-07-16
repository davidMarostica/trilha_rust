// struct Pedido {
//     nome_cliente: String,
//     valor: f32,
//     entregue: bool,
// }

// fn soma(x: i32, y: i32) -> impl Fn(i32) -> i32 {
//     let r = x + y;
//     move |multiplicador| r * multiplicador // closure
// }

// fn main() {
//     let resultado_soma = soma(5, 3); // 5 + 3 = 8
//     let resultado_final = resultado_soma(2); // 8 * 2 = 16

//     println!("Resultado da multiplicação: {}", resultado_final);

//     // Usando a struct Pedido para evitar warning
//     let pedidos = vec![
//         Pedido { nome_cliente: String::from("Alice"), valor: 150.0, entregue: true },
//         Pedido { nome_cliente: String::from("Bob"), valor: 250.0, entregue: false },
//     ];

//     // Closure inline para somar apenas pedidos entregues
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


// === Função de Alta Ordem =======

// fn main() {
//     let salario_bruto = 5000.0;

//     // Função de alta ordem que aplica descontos ao salário
//     fn aplicar_descontos(salario: f64, descontos: Vec<fn(f64) -> f64>) -> f64 {
//         let total_descontos = descontos.iter().fold(0.0, |valor_param, funcao_closure| valor_param + funcao_closure(salario));
//         salario - total_descontos
//     }

//     // Cálculo do salário líquido usando a função de alta ordem
//     let salario_liquido = aplicar_descontos(salario_bruto, vec![
//         |salario: f64| salario * 0.10, // Desconto do plano de saúde: 10%
//         |salario: f64| salario * 0.05, // Desconto do plano dentário: 5%
//         |salario: f64| salario * 0.03, // Desconto de vale-refeição: 3%
//     ]);

//     println!("Salário líquido: {:.2}", salario_liquido);
// }

// === Função de Alta Ordem =======
fn desconto_plano_saude(salario: f64) -> f64 {
    salario * 0.10 // Desconto de 10%
}

fn desconto_plano_dentario(salario: f64) -> f64 {
    salario * 0.05 // Desconto de 5%
}

fn desconto_vale_refeicao(salario: f64) -> f64 {
    salario * 0.03 // Desconto de 3%
}

// Função de alta ordem que aplica descontos ao salário
fn aplicar_descontos(salario: f64, descontos: Vec<fn(f64) -> f64>) -> f64 {
    let total_descontos = descontos.iter().fold(0.0, |valor_param, funcao_closure| valor_param + funcao_closure(salario));
    salario - total_descontos
}

fn main() {
    let salario_bruto = 5000.0;

    // Cálculo do salário líquido usando a função de alta ordem
    let salario_liquido = aplicar_descontos(salario_bruto, vec![
        desconto_plano_saude,
        desconto_plano_dentario,
        desconto_vale_refeicao,
    ]);

    println!("Salário líquido: {:.2}", salario_liquido);
}
