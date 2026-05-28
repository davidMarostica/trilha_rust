fn main() {
    let x: i32 = 5;
    println!("O valor de x e sua memória: {}, {:p}", x, & x);

    let x: i32 = x + 1; // Shadowing da variável x, agora com um novo valor
    println!("O valor de x e sua memória: {}, {:p}", x, & x);


    let x = x * 2 ; // Shadowing da variável x novamente, agora com um novo valor
    println!("O valor de x e sua memória: {}, {:p}", x, & x);

    
    println!("O valor de x é: {}", x);
}
