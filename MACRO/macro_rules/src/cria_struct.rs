// Definição da macro
macro_rules! cria_struct {
    (
        $nome_struct:ident {
            $($campo:ident: $tipo:ty),* $(,)?
        }
        $(fn $nome_metodo:ident(&$nome_metodo_struct:ident $(, $param_nome:ident: $param_tipo:ty)*) -> $ret_tipo:ty $corpo_metodo:block)*
    ) => {
        struct $nome_struct {
            $($campo: $tipo,)*
        }

        impl $nome_struct {
            $(
                fn $nome_metodo(&$nome_metodo_struct $(, $param_nome: $param_tipo)*) -> $ret_tipo $corpo_metodo
            )*
        }
    };
}

// Usando a macro para criar a struct Cliente
cria_struct! {
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
        nome: "João da Silva".to_string(),
        cpf: "123.456.789-00".to_string(),
    };

    println!("{}", cliente.mostra_nome());
    println!("{}", cliente.mostra_id());
    println!("{}", cliente.mais_numero_no_id(10));
}
