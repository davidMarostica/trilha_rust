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

macro_rules! cria_structs {
    // Caso para uma única struct sem campos
    ($nome:ident) => {
        #[derive(Debug)]
        struct $nome;
    };
    // Caso para uma struct com um campo
    ($nome:ident, $campo1:ident: $tipo1:ty) => {
        struct $nome {
            $campo1: $tipo1,
        }
    };
    // Caso para uma struct com dois campos
    ($nome:ident, $campo1:ident: $tipo1:ty, $campo2:ident: $tipo2:ty) => {
        struct $nome {
            $campo1: $tipo1,
            $campo2: $tipo2,
        }
    };
    // E assim por diante, você pode expandir para mais campos se necessário
}

fn main() {
    // Criando uma struct sem campos
    cria_structs!(Vazia);
    
    // Criando uma struct com um campo
    cria_structs!(UmCampo, campo1: u32);
    
    // Criando uma struct com dois campos
    cria_structs!(DoisCampos, campo1: u32, campo2: String);

    // Exemplo de uso
    let vazia = Vazia { };
    let item = UmCampo { campo1: 10 };
    let item2 = DoisCampos { campo1: 20, campo2: String::from("Olá") };
    
    println!("vazia: {:?}", vazia);
    println!("UmCampo: {}", item.campo1);
    println!("DoisCampos: {}, {}", item2.campo1, item2.campo2);
}