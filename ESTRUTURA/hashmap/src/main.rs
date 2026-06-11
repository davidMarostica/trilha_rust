use std::collections::HashMap;

// fn main() {
//     let mut dados: HashMap<String, i32> = HashMap::new();

//     dados.insert(String::from("Largura"), 10);
//     dados.insert(String::from("Altura"), 20);
//     dados.insert(String::from("Comprimento"), 30);

//     println!("{:?}", dados);

//     // Buscar pelo valor associado à chave "Largura"
//     if let Some(valor) = dados.get(&String::from("Largura")) {
//         println!("Valor encontrado: {}", valor);
//     } else {
//         println!("Valor não encontrado");
//     }
// }


// fn main() {
//     let mut contagem: HashMap<String, i32> = HashMap::new();

//     let palavras = vec!["azul", "vermelho", "azul", "verde", "vermelho"];

//     for palavra in palavras {
//         let contador = contagem.entry(palavra.to_string()).or_insert(0);
//         *contador += 1;
//     }

//     println!("{:?}", contagem);
// }

 //Iterando sobre chaves e valores
// fn main() {
//     let mut mapa = HashMap::new();
//     mapa.insert("Brasil", "Brasília");
//     mapa.insert("França", "Paris");
//     mapa.insert("Japão", "Tóquio");

//     for (pais, capital) in &mapa {
//         println!("A capital de {} é {}", pais, capital);
//     }
// }

 // Removendo valores

// fn main() {
//     let mut estoque = HashMap::new();
//     estoque.insert("Maçã", 50);
//     estoque.insert("Banana", 30);

//     println!("Antes: {:?}", estoque);

//     estoque.remove("Banana");

//     println!("Depois: {:?}", estoque);
// }


 // Usando tipos personalizados como chave

#[derive(Hash, Eq, PartialEq, Debug)]
struct Ponto {
    x: i32,
    y: i32,
}

fn main() {
    let mut mapa: HashMap<Ponto, String> = HashMap::new();

    mapa.insert(Ponto { x: 0, y: 0 }, "Origem".to_string());
    mapa.insert(Ponto { x: 1, y: 2 }, "Outro ponto".to_string());

    println!("{:?}", mapa);
}
