/* 
fn main() {
    println!("Olá, mundo!");

    outra_funcao();
}

fn outra_funcao() {
    println!("Outra função.");
}
*/

// fn main() {

//     let n1: i16 = 10;
//     let n2: i16 = 20;

//     let r: i16 = soma(  n1, n2 );
//     println!("O resultado da soma é: {}", r);
// }

// fn soma(x:i16, y:i16) -> i16 {
//    x + y    
// }

// fn main() {
//     let resultado = retorna_string(10);
//     println!("O resultado é: {}", resultado);
// }

// fn retorna_string(param: i32) -> String {
//     if param == 10 {
//         String::from("O parâmetro é igual a 10")
//     } else {
//         String::from("O parâmetro é diferente de 10")
//     }
// }


fn main() {
   let r: i32 = mostrar_na_tela(1); 

   println!("valor somado é: {}", r);
}

fn mostrar_na_tela(i: i32) -> i32 {
    if i > 10 {
        return i;
    }
    println!("valor de i é {}", i);
    mostrar_na_tela(i + 1)
}
