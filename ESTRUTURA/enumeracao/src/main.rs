enum Tipo {
    Juridica,
    Fisica,
}

struct Pessoa {
    nome: String,
    documento: String,
    tipo: Tipo,
}

fn main() {
    let david: Pessoa = Pessoa {
        nome: String::from("david"),
        documento: String::from("12.456.789/9999-00"),
        tipo: Tipo::Juridica,
    };

    let maria: Pessoa = Pessoa {
        nome: String::from("Maria"),
        documento: String::from("123.456.789-00"),
        tipo: Tipo::Fisica,
    };

    match david.tipo {
        Tipo::Juridica => println!("Pessoa jurídica: {} - Documento: {}", david.nome, david.documento),
        Tipo::Fisica => println!("Pessoa física: {} - Documento: {}", david.nome, david.documento),
    }

    match maria.tipo {
        Tipo::Juridica => println!("Pessoa jurídica: {} - Documento: {}", maria.nome, maria.documento),
        Tipo::Fisica => println!("Pessoa física: {} - Documento: {}", maria.nome, maria.documento),
    };
}
