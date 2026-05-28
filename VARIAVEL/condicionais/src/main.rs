/* 
fn main() {
    let numero = 3;

    if numero < 5 {
        println!("condição era verdadeira");
    } else {
        println!("condição era falsa");
    }
}
*/
/* 
fn main() {
    let numero = 6;

    if numero % 4 == 0 {
        println!("número é divisível por 4");
    } else if numero % 3 == 0 {
        println!("número é divisível por 3");
    } else if numero % 2 == 0 {
        println!("número é divisível por 2");
    } else {
        println!("número não é divisível por 4, 3 ou 2");
    }
}
*/

/* 
fn main() {
    let numero : i32 = 3;

    let resultado: i32 = if numero < 5 {
        numero * 2
    } else {
        numero / 2
    };
    println!("{}", resultado);
}  
*/
/* 
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut indice = 0;

    while indice < 5 {
        println!("O valor é: {}", a[indice]);

        indice = indice + 1;
    }
}
*/
 fn main() {
    let mut x : i32 = 1;
    while x <= 100 {
        if x >= 50 && x <= 60 {
            x += 1;
            continue;
        }
        println!("O valor de x é: {}", x);
        x += 1;
            

              
    }
}
   
