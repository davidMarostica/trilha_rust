// fn main() {
//    // memória stack (variavel do tipo copy no rust)
//    let x: i32 = 10;
//    let y: i32 = x; // copia o valor de x para y

//     println!("Valor de x: {}", x);
//     println!("Valor de y: {}", y);
// }

// fn main() {


//    let x: i32 = 10;
//    let y: i32 = x; // copia o valor de x para y

//     println!("Valor de x: {} - Referência: {:p}", x, &x);
//     println!("Valor de y: {} - Referência: {:p}", y, &y);
// }

// fn main() {
//    // memória stack (variavel do tipo copy no rust)
//    let x: i32 = 10;
//    let y: &i32 = &x; // y é uma referência para x, não uma cópia

//     println!("Valor de x: {} - Referência: {:p}", x, &x);
//     println!("Valor de y: {} - Referência: {:p}", y, &y);
// }

// fn main() {
//    let x: i32 = 10;
//    let y: i32 = x; 
//     println!("Valor de x é {} ", x);
//     println!("Valor de y é {} ", y);

//     // Improme o endereço de memória de x e y, mostrando que são diferentes, mesmo que os valores sejam iguais

//     println!("Endereço de memória de x: {:p}", &x);
//     println!("Endereço de memória de y: {:p}", &y);
// }


fn main() {
   let x: i32 = 10; // owner da variável x
   let y: &i32 = &x; 
    println!("Valor de x é {} ", x);
    println!("Valor de y é {} ", y);

    // Imprime o endereço de memória de x e y, mostrando que são diferentes, mesmo que os valores sejam iguais

    println!("Endereço de memória de x: {:p}", &x); // {:p} é usado para imprimir o endereço de memória
    println!("Endereço de memória de y: {:p}", &y); // y é uma referência para x, então o endereço de y é diferente do endereço de x, mas o valor apontado por y é o mesmo que o valor de x

    let t: &i32 = y; // cria outra refereucia para o dono x ;
    println!("Endereço de mempria de t: {:p} ", &t);

    let w: i32 = *y; // w Desreferencia y para obter o valor de x e atribui a w, w é uma cópia do valor de x    
    println!("Endereço de memória de w: {:p}", &w); 
}

/* 

x → endereço do valor real.

y → endereço da variável que guarda a referência.

t → outro endereço de referência (também aponta para x).

w → endereço de uma nova variável com o mesmo valor de x.
*/