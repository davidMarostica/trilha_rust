mod enums;
mod models;

use crate::enums::{Tipo, Sexo};
use crate::models::Pessoa;

fn main() {
    let daniel: Pessoa = Pessoa::new("Daniel", "123456789", Tipo::Fisica);
    daniel.show();

    println!("{}", "-".repeat(20)); // Separador

    let c_e_c: Pessoa = Pessoa::new("C & C", "987654321", Tipo::Juridica);
    c_e_c.show();

    println!("{}", "-".repeat(20)); // Separador

    let sexo_f: Sexo = Sexo::Feminino;
    let sexo_m: Sexo = Sexo::Masculino;
    let sexo_o: Sexo = Sexo::Outros;

    println!("{}", sexo_string(sexo_f));
    println!("{}", sexo_string(sexo_m));
    println!("{}", sexo_string(sexo_o));
}

fn sexo_string(sexo: Sexo) -> &'static str {
    match sexo {
        Sexo::Masculino => "Masculino",
        Sexo::Feminino => "Feminino",
        Sexo::Outros => "Outros",
    }
}
