//Array de tuplas
// fn main() {
//     // Array fixo de tuplas (pares coordenadas)
//     let pontos: [(i32, i32); 3] = [(0, 0), (10, 5), (20, 15)];

//     for (x, y) in pontos {
//         println!("Ponto: ({}, {})", x, y);
//     }
// }

//Tupla dentro de array multidimensional

// fn main() {
//     // Array de tuplas aninhadas
//     let dados: [((&str, i32), (bool, &str)); 2] = [
//         (("David", 35), (true, "Engenheiro")),
//         (("Maria", 28), (false, "Designer")),
//     ];

//     for ((nome, idade), (ativo, profissao)) in dados {
//         println!("{} ({} anos) - {} | Ativo: {}", nome, idade, profissao, ativo);
//     }
// }


// Função que retorna array de tuplas

// fn gerar_pontos() -> [(i32, i32); 4] {
//     [(0, 0), (5, 10), (10, 20), (15, 30)]
// }

// fn main() {
//     let pontos = gerar_pontos();

//     for (x, y) in pontos {
//         println!("Coordenada: ({}, {})", x, y);
//     }
// }


//array mutável

// fn main() {
//     let mut numeros: [i32; 5] = [1, 2, 3, 4, 5];

//     // Alterando elementos específicos
//     numeros[0] = 100;
//     numeros[4] = 999;

//     println!("Array atualizado: {:?}", numeros);
// }

// fn main() {
//     let mut estoque: [(&str, i32); 3] = [
//         ("Notebook", 10),
//         ("Mouse", 50),
//         ("Teclado", 30),
//     ];

//     // Atualizando o estoque
//     for item in estoque.iter_mut() {
//         item.1 -= 1; // reduz estoque de cada produto
//     }

//     // Mostrando todos os itens
//     for (produto, qtd) in estoque {
//         println!("{} | Quantidade: {}", produto, qtd);
//     }

//     // Mostrando a quantidade total de elementos no array
//     println!("Total de produtos cadastrados: {}", estoque.len());
// }


fn main() {
    // Criando vetor com 10 itens
    let mut estoque: Vec<(&str, i32)> = vec![
        ("Notebook", 10),
        ("Mouse", 50),
        ("Teclado", 30),
        ("Monitor", 15),
        ("Impressora", 8),
        ("HD Externo", 20),
        ("Pendrive", 100),
        ("Cadeira Gamer", 5),
        ("Mesa Escritório", 12),
        ("Headset", 25),
    ];

    println!("Estoque inicial ({} itens):", estoque.len());
    for (produto, qtd) in &estoque {
        println!("{} | Quantidade: {}", produto, qtd);
    }

    // Adicionando 2 novos itens
    estoque.push(("Webcam", 18));
    estoque.push(("Microfone", 10));

    println!("\nApós adicionar 2 itens ({} itens):", estoque.len());
    for (produto, qtd) in &estoque {
        println!("{} | Quantidade: {}", produto, qtd);
    }

    // Removendo 3 últimos itens
    for _ in 0..3 {
        if let Some(removido) = estoque.pop() {
            println!("\nRemovido: {} | Quantidade: {}", removido.0, removido.1);
        }
    }

    println!("\nEstoque final ({} itens):", estoque.len());
    for (produto, qtd) in &estoque {
        println!("{} | Quantidade: {}", produto, qtd);
    }
}
