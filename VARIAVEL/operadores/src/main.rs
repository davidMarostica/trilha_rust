// fn main() {
//     let remainder: i32 = 43 % 5;
//     println!("O resto da divisão de 43 por 5 é: {}", remainder);
// }

// fn main() {
//    let equal: bool = 5 == 5;
//    let not_equal: bool = 5 != 3;

//     println!("5 é igual a 5? {}", equal);
//     println!("5 é diferente de 3? {}", not_equal);
// }


// Operadores lógicos
// fn main() {    let a: bool = true;
//     let b: bool = false;
//     let and_result: bool = a && b;
//     let or_result: bool = a || b;
//     let not_result: bool = !a;

//     println!("{} && {}: {}", a, b, and_result);
//     println!("{} || {}: {}", a, b, or_result);
//     println!("!{}: {}", a, not_result);
// }

// operador de atribuição
//  fn main() {
//     let mut x: i32 = 10;
//     println!("Valor inicial de x: {}", x);

//     x += 5; // Equivalente a x = x + 5
//     println!("Após x += 5: {}", x);

//     x -= 3; // Equivalente a x = x - 3
//     println!("Após x -= 3: {}", x);

//     x *= 2; // Equivalente a x = x * 2
//     println!("Após x *= 2: {}", x);

//     x /= 4; // Equivalente a x = x / 4
//     println!("Após x /= 4: {}", x);
//  }


//Operador bitwise
// fn main() {    let a: u8 = 0b10101010; // 170 em decimal    

//     let b: u8 = 0b11001100; // 204 em decimal
//     let and_result: u8 = a & b;
//     let or_result: u8 = a | b;
//     let xor_result: u8 = a ^ b;

//     println!("{} & {}: {}", a, b, and_result);
//     println!("{} | {}: {}", a, b, or_result);
//     println!("{} ^ {}: {}", a, b, xor_result);
// }


// Operadores de comparação
fn main() {    let x: i32 = 10;
    let y: i32 = 20;    

    println!("x == y: {}", x == y); // false
    println!("x != y: {}", x != y); // true
    println!("x > y: {}", x > y);   // false
    println!("x < y: {}", x < y);   // true
    println!("x >= y: {}", x >= y); // false    
    println!("x <= y: {}", x <= y); // true
}