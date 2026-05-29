// fn main() {
//     // variável na memória heap
//     let s1: String = String::from("Olá, mundo!");
//     let s2: String = s1; // move o valor de s1 para s2

//     println!("s2: {} - referência: {:p}", s2, &s2);
// }

// fn main() {
//     // variável na memória heap
//     let s1: String = String::from("Olá, mundo!");
//     let s2: String = s1.clone(); // clona o valor de s1 para s2

//     println!("s1: {} - referência: {:p}", s1, &s1);
//     println!("s2: {} - referência: {:p}", s2, &s2);
// }

// fn main() {
//     // variável na memória heap
//     let s1: String = String::from("Olá, mundo!");
//     let s2: String = s1.clone(); // clona o valor de s1 para s2

//     println!("String s1: {} - referência: {:p}", s1, &s1);
//     println!("String s2: {} - referência: {:p}", s2, &s2);

//     let s3: &str = "Olá, mundo!"; // string literal na memória estática
//     let s4: &str = s3; // copia a referência de s3 para s4
    
//     println!("&str s3: {} - referência: {:p}", s3, &s3);
//     println!("&str s4: {} - referência: {:p}", s4, &s4);
// }

// fn main() {
//     // variável na memória heap
//     let mut s1: String = String::from("Olá, mundo!");
//     s1.push_str("-teste"); // modifica o valor de s1

//     let s2: String = s1.clone(); // clona o valor de s1 para s2

//     println!("String s1: {} - referência: {:p}", s1, &s1);
//     println!("String s2: {} - referência: {:p}", s2, &s2);

//     let s3: &str = "Olá, mundo!"; // string literal na memória estática
    
//     let s4: String = format!("{}-teste", s3); // cria uma nova String a partir de s3

    
//     println!("&str s3: {} - referência: {:p}", s3, &s3);
//     println!("&str s4: {} - referência: {:p}", s4, &s4);
// }

// fn main() {
//     let original_string: String = String::from("Rust e uma linguagem de programação incrível!");

//     // Criando uma substring usando slicing
//     let substring: &str = &original_string[0..4]; // Obtendo os primeiros 4 caracteres

//     println!("String original: {} - referência: {:p}", original_string, &original_string);
//     println!("Substring: {} - referência: {:p}", substring, &substring);
// }



fn main() {
    // Convertendo String em &str usando as_str()
    let s1: String = String::from("Olá, mundo!");
    let referencia_to_s1: &str = s1.as_str(); // obtendo uma referência &str a partir de s1

    // Convertendo String em &str fazendo uma referência direta
    let s2: String = String::from("Rust é incrível!");
    let referencia_to_s2: &str = &s2; // obtendo uma referência

    println!("String s1: {} - referência: {:p}", s1, &s1);
    println!("&str referencia_to_s1: {} - referência: {:p}", referencia_to_s1, &referencia_to_s1);  

    println!("String s2: {} - referência: {:p}", s2, &s2);
    println!("&str referencia_to_s2: {} - referência: {:p}", referencia_to_s2, &referencia_to_s2);
}
