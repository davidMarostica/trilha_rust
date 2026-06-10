// struct Pessoa {
//     nome: String,
//     idade: u8,
// }

// fn main() {
//     let pessoa: Pessoa = Pessoa {
//         nome: String::from("David"),
//         idade: 43,
//     };

//     // Desestruturação de uma struct
//     let Pessoa { nome, idade } = pessoa;
//     println!("Nome: {}, Idade: {}", nome, idade);
// }


//  // Desestruturação de Arrays e Tuplas
// fn main() {
//     // Declarando um array de 5 elementos
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];

//     // Desconstrução: pegando os dois primeiros e ignorando o resto
//     let [primeiro, segundo, ..] = arr;

//     println!("Primeiro: {}, Segundo: {}", primeiro, segundo);
// }

// // Desestruturação uma tupla
// fn main() {
//     let tupla: (&str, i32, f32) = ("David", 2024, 3.14);   

//     let (linguagem, ano, ..) = tupla; // Desestruturação da tupla
//     let quantidade: usize = linguagem.len(); // Usando a variável 'linguagem' para obter o comprimento da string
//     println!("Linguagem: {}, Ano: {}, Tamanho: {}", linguagem, ano, quantidade);

// }

// Desestruturação de Enums
// enum Mensagem {
//     Enviar { id: u32, testo: String },
//     Receber { id: u32, testo: String },
// }

// fn main() {
//     // Exemplo usando a variante Enviar
//     let mensagem1 = Mensagem::Enviar { id: 1, texto: String::from("Olá, mundo!") };

//     // Exemplo usando a variante Receber
//     let mensagem2 = Mensagem::Receber { id: 2, texto: String::from("Oi!") };

//     // Match para tratar cada mensagem
//     match mensagem1 {
//         Mensagem::Enviar { id, texto } => println!("Enviando mensagem com ID {}: {}", id, texto),
//         Mensagem::Receber { id, texto } => println!("Recebendo mensagem com ID {}: {}", id, texto),
//     }

//     match mensagem2 {
//         Mensagem::Enviar { id, texto } => println!("Enviando mensagem com ID {}: {}", id, texto),
//         Mensagem::Receber { id, texto } => println!("Recebendo mensagem com ID {}: {}", id, texto),
//     }
// }



struct Pessoa {
    nome: String,
    sobrenome: String,
    idade: u8,
}

fn main() {
    let pessoa = Pessoa {
        nome: String::from("David"),
        sobrenome: "Teste".to_string(),
        idade: 43,
    };

    let Pessoa { nome, sobrenome, idade } = pessoa;

    let pessoa2 = Pessoa {
        nome: nome.to_owned() + " Silva", 
        sobrenome: sobrenome.to_owned(),
        idade,
    };
    
    println!("Pessoa 1: Nome: {}, Sobrenome: {}, Idade: {}", nome, sobrenome, idade);
    println!("Pessoa 2: Nome: {}, Sobrenome: {}, Idade: {}", pessoa2.nome, pessoa2.sobrenome, pessoa2.idade);
}
