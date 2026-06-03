#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    // Método para calcular a área
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

#[derive(Debug)]
struct Cliente {
    id: u32,
    nome: String,
    email: String,
    cpf: String,
}

impl Cliente {
    // Construtor "new"
    fn new(id: u32, nome: String, email: String, cpf: String) -> Cliente {
        Cliente { id, nome, email, cpf }
    }

    // Método para validar CPF
    fn cpf_valido(&self) -> bool {
        self.cpf.len() == 14
            && self
                .cpf
                .chars()
                .all(|c| c.is_ascii_digit() || c == '.' || c == '-')
    }
}

fn main() {
    let cliente1 = Cliente::new(
        1,
        String::from("david silva"),
        String::from("david@example.com"),
        String::from("123.456.789-00"),
    );

    println!("Cliente: {:?}", cliente1);
    println!("CPF válido? {}", cliente1.cpf_valido());
    println!("ID do cliente: {}", cliente1.id);
    println!("Nome do cliente: {}", cliente1.nome);
    println!("Email: {}", cliente1.email);

    let rect = Rectangle { length: 10, width: 5 };
    println!("Retângulo: {:?}", rect);
    println!("Área do retângulo: {}", rect.area());
}
