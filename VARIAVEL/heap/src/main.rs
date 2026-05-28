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

fn main() {
    // variável na memória heap
    let s1: String = String::from("Olá, mundo!");
    let s2: String = s1.clone(); // clona o valor de s1 para s2

    println!("String s1: {} - referência: {:p}", s1, &s1);
    println!("String s2: {} - referência: {:p}", s2, &s2);

    let s3: &str = "Olá, mundo!"; // string literal na memória estática
    let s4: &str = s3; // copia a referência de s3 para s4
    
    println!("&str s3: {} - referência: {:p}", s3, &s3);
    println!("&str s4: {} - referência: {:p}", s4, &s4);
}