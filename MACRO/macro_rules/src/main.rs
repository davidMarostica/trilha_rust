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

trait ExemploTrait {
    fn metodo_exemplo(&self);
}

// Criando uma macro para implementar automaticamente o trait para qualquer struct
macro_rules! implementa_trait {
    ($t:ty) => {
        impl ExemploTrait for $t {
            fn metodo_exemplo(&self) {
                println!("Método exemplo chamado para {:?}", self);
            }
        }
    };
}

// Definindo uma structclar
#[derive(Debug)]
struct MinhaStruct;

#[derive(Debug)]
struct MinhaStruct2;

// Usando a macro para aplicar a implementação do trait à struct
implementa_trait!(MinhaStruct);
implementa_trait!(MinhaStruct2);

fn main() {
    let minha_instancia = MinhaStruct;
    minha_instancia.metodo_exemplo();


    let minha_instancia2 = MinhaStruct2;
    minha_instancia2.metodo_exemplo();
}