////// ===== Operador Option ======


// fn encontrar_divisor(numero: i32) -> Option<i32> {
//     if numero % 2 == 0 {
//         Some(2) // Um divisor encontrado
//     } else if numero % 3 == 0 {
//         Some(3) // Outro divisor encontrado
//     } else {
//         None // Nenhum divisor encontrado
//     }
// }

// fn main() {
//     let numero = 1;
//     match encontrar_divisor(numero) {
//         Some(divisor) => println!("Divisor encontrado: {}", divisor),
//         None => println!("Nenhum divisor encontrado para {}", numero),
//     }
// }

/////// ===== Operador Result ======

// Enum Result já faz parte da biblioteca padrão do Rust
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// fn calcular_raiz_quadrada(numero: f64) -> Result<f64, String> {
//     if numero >= 0.0 {
//         Ok(numero.sqrt()) // Retorna a raiz quadrada do número
//     } else {
//         Err(String::from("número negativo não possui raiz quadrada real")) // Retorna um erro
//     }
// }

// fn calcular_raiz_quadrada2(numero: f64) -> Result<f64, String> {
//     let r = calcular_raiz_quadrada(numero)?; // Se Err, retorna imediatamente
//     Ok(r) // Se Ok, retorna o valor
// }

// fn main() {
//     let numero = -4.0;

//     // Usando a primeira função
//     match calcular_raiz_quadrada(numero) {
//         Ok(raiz) => println!("A raiz quadrada de {} é {}", numero, raiz),
//         Err(e) => println!("Erro: {}", e),
//     }

//     // Usando a segunda função com operador ?
//     match calcular_raiz_quadrada2(numero) {
//         Ok(raiz) => println!("Versão 2: raiz quadrada de {} é {}", numero, raiz),
//         Err(e) => println!("Versão 2: erro -> {}", e),
//     }
// }


/////// ===== io::Error ======

// use std::fs::File;
// use std::io;
// use std::io::Read;

// fn ler_arquivo(nome: &str) -> Result<String, io::Error> {
//     let mut f = File::open(nome)?; // Se não conseguir abrir, retorna Err(io::Error)
//     let mut s = String::new();
//     f.read_to_string(&mut s)?; // Se não conseguir ler, retorna Err(io::Error)
//     Ok(s) // Se tudo deu certo, retorna o conteúdo
// }

// fn main() {
//     match ler_arquivo("meu_arquivo.txt") {
//         Ok(conteudo) => println!("Conteúdo do arquivo: {}", conteudo),
//         Err(e) => println!("Falha ao abrir o arquivo: {}", e),
//     }
// }

// //// ========= ? = retorna o erro para o chamador =====

// use std::fs::File;
// use std::io::{self, Read};

// // Função que tenta ler o conteúdo de um arquivo para uma String
// fn ler_conteudo_arquivo(nome_arquivo: &str) -> Result<String, io::Error> {
//     let mut f = File::open(nome_arquivo)?; // Se falhar, retorna o erro para o chamador
//     let mut conteudo = String::new();
//     f.read_to_string(&mut conteudo)?; // Se falhar, retorna o erro para o chamador
//     Ok(conteudo) // Retorna o conteúdo do arquivo em caso de sucesso
// }

// fn main() -> Result<(), io::Error> {
//     let nome_arquivo = "exemplo.txt";
//     let conteudo = ler_conteudo_arquivo(nome_arquivo)?; // Propaga erro se houver
//     println!("Conteúdo do arquivo:\n{}", conteudo);
//     Ok(())
// }

// =========== Error Types Customizados com Trait Debug ============

use regex::Regex;
use std::error::Error;
use std::fmt;


#[derive(Debug)]
enum ValidationError {
    EmptyName(String),
    NonUniqueName(String),
    InvalidFormat(String),
}

// Implementando Display para o nosso erro customizado
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Para que ValidationError possa ser tratado como um erro
impl Error for ValidationError {}

fn validar_nome(nome: &str, nomes_existentes: &[&str]) -> Result<(), Box<dyn Error>> {
    // Validar se o nome não está vazio
    if nome.is_empty() {
        return Err(Box::new(ValidationError::EmptyName("O nome não pode ser vazio".to_string())));
    } else if nomes_existentes.contains(&nome) {
        return Err(Box::new(ValidationError::NonUniqueName("O nome deve ser unico".to_string())));
    }

    // Validar o formato do nome com regex
    let regex = Regex::new(r"^[a-zA-Z\s]+$").unwrap();
    // Exemplos de Strings Válidas na REGEX
    //     "Alice"
    //     "Bob Smith"
    //     "a b c"
    //     "Z"

    if !regex.is_match(nome) {
        return Err(Box::new(ValidationError::InvalidFormat("O nome não está no padrão permitido".to_string())));
    }

    Ok(())
}

fn main() {
    let nomes_existentes = vec!["Alice", "Bob"];
    match validar_nome("Bob", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => println!("Erro de validação: {}", e),
    }

    match validar_nome("", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => println!("Erro de validação: {}", e),
    }

    match validar_nome("123 Danilo", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => println!("Erro de validação: {}", e),
    }

    match validar_nome("Danilo", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => println!("Erro de validação: {}", e),
    }

    match validar_nome("Alice", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => {
            match e.downcast_ref::<ValidationError>().unwrap() {
                ValidationError::EmptyName(erro) => println!("Erro de validação: o nome não pode ser vazio - {}", erro),
                ValidationError::NonUniqueName(erro) => {
                    println!("{}", erro);
                    println!("Erro de validação: o nome não é único");
                    println!("Por favor, escolha um nome diferente.");
                },
                ValidationError::InvalidFormat(erro) => println!("Erro de validação: o formato do nome é inválido - {}", erro),
            }
        },
    }
}
