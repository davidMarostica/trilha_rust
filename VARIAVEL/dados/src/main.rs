fn main() {
    let x: i8 = 1; // 8-bit signed integer
    println!("O valor de x é: {}", x);

    let x: i16 = 9999; // 16-bit signed integer
    println!("O valor de x é: {}", x);

    let x: f32 = 9.0; // 32-bit floating point number
    println!("O valor de x é: {}", x);

    let x: char = 'A'; // character type
    println!("O valor de x é: {}", x);

    let x: &str = "Hello, world!"; // string slice type
    println!("O valor de x é: {}", x);

    let x: &str = "ts"; // string slice type
    println!("O valor de x é: {}", x);

    let mut x: i32 = -1; // mutable variable of type 32-bit signed integer
    x += 6 ; // modifying the value of x
    println!("O valor de x é: {}", x);
}
