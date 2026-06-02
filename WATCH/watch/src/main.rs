fn main() {
    let x: String = String::from("Depurando o código");
    let x_modificarda: String = mostrar_na_tela_alterada(x);
    print!("========================================================\n");
    println!("Oláaa !!! - {}", x_modificarda);
    print!("========================================================\n");
}

fn mostrar_na_tela_alterada(mut str: String) -> String {
    str += " - O código foi alterado";
    str
}
