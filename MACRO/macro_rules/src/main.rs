/////// ======= Macro declarativa simples =======

// macro_rules! diz_ola {
//     () => {
//         println!("Olá, mundo!")
//     };
// }

// fn main() {
//     diz_ola!()
// }


// ======= Macro declarativa com trait =======

// Definindo um trait com métodos que queremos forçar a implementação

// trait ExemploTrait {
//     fn metodo_exemplo(&self);
// }

// // Criando uma macro para implementar automaticamente o trait para qualquer struct
// macro_rules! implementa_trait {
//     ($t:ty) => {
//         impl ExemploTrait for $t {
//             fn metodo_exemplo(&self) {
//                 println!("Método exemplo chamado para {:?}", self);
//             }
//         }
//     };
// }

// // Definindo uma structclar
// #[derive(Debug)]
// struct MinhaStruct;

// #[derive(Debug)]
// struct MinhaStruct2;

// // Usando a macro para aplicar a implementação do trait à struct
// implementa_trait!(MinhaStruct);
// implementa_trait!(MinhaStruct2);

// fn main() {
//     let minha_instancia = MinhaStruct;
//     minha_instancia.metodo_exemplo();


//     let minha_instancia2 = MinhaStruct2;
//     minha_instancia2.metodo_exemplo();
// }


///// ======= Macro com metaprogramação para criar structs =======

// macro_rules! cria_structs {
//     // Caso para uma única struct sem campos
//     ($nome:ident) => {
//         #[derive(Debug)]
//         struct $nome;
//     };
//     // Caso para uma struct com um campo
//     ($nome:ident, $campo1:ident: $tipo1:ty) => {
//         struct $nome {
//             $campo1: $tipo1,
//         }
//     };
//     // Caso para uma struct com dois campos
//     ($nome:ident, $campo1:ident: $tipo1:ty, $campo2:ident: $tipo2:ty) => {
//         struct $nome {
//             $campo1: $tipo1,
//             $campo2: $tipo2,
//         }
//     };
//     // E assim por diante, você pode expandir para mais campos se necessário
// }

// fn main() {
//     // Criando uma struct sem campos
//     cria_structs!(Vazia);
    
//     // Criando uma struct com um campo
//     cria_structs!(UmCampo, campo1: u32);
    
//     // Criando uma struct com dois campos
//     cria_structs!(DoisCampos, campo1: u32, campo2: String);

//     // Exemplo de uso
//     let vazia = Vazia { };
//     let item = UmCampo { campo1: 10 };
//     let item2 = DoisCampos { campo1: 20, campo2: String::from("Olá") };
    
//     println!("vazia: {:?}", vazia);
//     println!("UmCampo: {}", item.campo1);
//     println!("DoisCampos: {}, {}", item2.campo1, item2.campo2);
// }


///// ======= Macro com metaprogramação para criar metodos de uma Struct =======

// // Definindo uma macro para implementar métodos especificados em uma struct.
// macro_rules! implementa_metodos {
//     // Aceita o nome da struct seguido por uma sequência de identificadores (nomes de métodos).
//     ($struct:ident, $($metodo:ident),*) => {
//         impl $struct {
//             // Para cada identificador fornecido, gera um método que imprime uma mensagem.
//             $(
//                 fn $metodo(&self) {
//                     println!("{}::{} foi chamado", stringify!($struct), stringify!($metodo));
//                 }
//             )*
//         }
//     };
// }

// // Definindo uma struct de exemplo.
// struct ExemploStruct;

// // Usando a macro para adicionar métodos à struct `ExemploStruct`.
// implementa_metodos!(ExemploStruct, metodo_a, metodo_b, metodo_c);

// fn main() {
//     let exemplo = ExemploStruct;
    
//     // Chamando os métodos gerados pela macro.
//     exemplo.metodo_a();
//     exemplo.metodo_b();
//     exemplo.metodo_c();
// }


//// ====== Macro para criar atributos =======

// macro_rules! define_struct_com_atributos {
//     // A macro aceita o nome da struct seguido por uma lista de pares (nome do atributo: tipo do atributo)
//     ($nome:ident, $($campo:ident: $tipo:ty),*) => {
//         struct $nome {
//             // Gera um campo para cada par nome:tipo fornecido
//             $(
//                 $campo: $tipo,
//             )*
//         }
//     };
// }

// // Usando a macro para definir uma nova struct com atributos especificados
// define_struct_com_atributos!(
//     Pessoa,
//     nome: String,
//     idade: u8,
//     email: String
// );

// fn main() {
//     // Criando uma instância da struct Pessoa
//     let pessoa = Pessoa {
//         nome: String::from("João"),
//         idade: 30,
//         email: String::from("joao@email.com"),
//     };

//     // Exemplo de acesso aos campos
//     println!("Nome: {}", pessoa.nome);
//     println!("Idade: {}", pessoa.idade);
//     println!("Email: {}", pessoa.email);
// }

/// ========== Exemplo JSON =========

// use serde::{Serialize, Deserialize};
// use serde_json::Result;
// use std::fs;


// macro_rules! cria_struct {
//     ($nome:ident { $($campo:ident: $tipo:ty),* $(,)? }) => {
//         #[derive(Debug, Serialize, Deserialize)]
//         struct $nome {
//             $($campo: $tipo),*
//         }
//     };
// }

// // Exemplo de uso da macro para criar uma struct com base nos campos especificados
// cria_struct! {
//     Cliente {
//         id: u32,
//         nome: String,
//         cpf: String,
//     }
// }

// fn main() -> Result<()> {
//     // Lendo o arquivo JSON
//     let data = fs::read_to_string("clientes.json").expect("Falha ao ler arquivo");

//     // Deserializando o JSON para um Vec<Cliente>
//     let clientes: Vec<Cliente> = serde_json::from_str(&data)?;

//     // Iterando sobre os clientes e imprimindo seus dados
//     for cliente in clientes {
//         println!("{:?}", cliente);
//     }

//     Ok(())
// }


// ///// ========== Usando Cria struct com metodo de uma crate =========


#[macro_use]
extern crate minha_macro_lib2;

cria_struct_crate_lib! {
    Cliente {
        id: u32,
        nome: String,
        cpf: String,
    }

    fn mostra_nome(&self) -> String {
        format!("Nome: {}", self.nome)
    }

    fn mostra_id(&self) -> String {
        format!("ID: {}", self.id)
    }

    fn mais_numero_no_id(&self, numero: u32) -> String {
        format!("ID + Numero: {}", self.id + numero)
    }
}

fn main() {
    let cliente = Cliente {
        id: 1,
        nome: "David Aparecido da Silva".to_string(),
        cpf: "123.456.789-00".to_string(),
    };

    println!("{}", cliente.mostra_nome());
    println!("{}", cliente.mostra_id());
    println!("{}", cliente.mais_numero_no_id(10));
}
