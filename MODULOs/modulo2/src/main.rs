mod enums;
mod models;

use crate::enums::{tipo::Tipo, sexo::Sexo};
use crate::models::pessoa::Pessoa;
use modulo2::mostra_oi;

fn main() {
    let david = Pessoa::new("David", "123456789", Tipo::Fisica, Sexo::Masculino);
    david.show();

    println!("{}", "-".repeat(20));

    let empresa = Pessoa::new("C & C", "987654321", Tipo::Juridica, Sexo::Outros);
    empresa.show();

    println!("{}", "-".repeat(20));

    mostra_oi();

    println!("Sexo F: {}", Sexo::Feminino.descricao());
    println!("Sexo M: {}", Sexo::Masculino.descricao());
    println!("Sexo O: {}", Sexo::Outros.descricao());
}
