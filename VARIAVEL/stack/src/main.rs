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


// fn main() {
//    let x: i32 = 10; // owner da variável x
//    let y: &i32 = &x; 
//     println!("Valor de x é {} ", x);
//     println!("Valor de y é {} ", y);

//     // Imprime o endereço de memória de x e y, mostrando que são diferentes, mesmo que os valores sejam iguais

//     println!("Endereço de memória de x: {:p}", &x); // {:p} é usado para imprimir o endereço de memória
//     println!("Endereço de memória de y: {:p}", &y); // y é uma referência para x, então o endereço de y é diferente do endereço de x, mas o valor apontado por y é o mesmo que o valor de x

//     let t: &i32 = y; // cria outra refereucia para o dono x ;
//     println!("Endereço de mempria de t: {:p} ", &t);

//     let w: i32 = *y; // w Desreferencia y para obter o valor de x e atribui a w, w é uma cópia do valor de x    
//     println!("Endereço de memória de w: {:p}", &w); 
// }

/* 

x → endereço do valor real.

y → endereço da variável que guarda a referência.

t → outro endereço de referência (também aponta para x).

w → endereço de uma nova variável com o mesmo valor de x.
*/



// fn main() {
//    let x: i32 = 10; 
//    let y: &i32 = &x; 
   
//     imprime_valor(&x);
//     imprime_valor(y); // y é uma referência para x, então podemos passar y diretamente
// }

// fn imprime_valor(valor: &i32) {
//     println!("Valor: {} - Endereço de memória: {:p}", valor, valor);
// }

fn main() {
   let mut x : i32 = 10; 
   
   imprime_valor(&mut x); // Passando uma referência  mutável para x
   println!("[Original]Valor de x após a modificação: {} - referencia: {:p}", x, &  x);

   imprime_valor(&mut x); // Passando a referência mutável novamente para modificar o valor de x
   println!("Valor de x após a segunda modificação: {} - referencia: {:p}", x, &x);
}


   fn imprime_valor(valor: &mut i32) {
       *valor += 55; // Modificando o valor apontado pela referência
       println!("[Reborrowing]Valor modificado: {} - Endereço de memória: {:p}", valor, valor);
   }

