// Exemplo de tupla
fn exemplo_tupla() {
    let funcionario: (i32, String, f32) = (42, String::from("david"), 30000.0);
    println!("==========Exemplo de tupla:==========");
    println!("Funcionário: {:?}", funcionario);
    println!("ID: {}", funcionario.0);
    println!("Nome: {}", funcionario.1);
    println!("Salário: {}", funcionario.2);
}

// Exemplo de arrays
fn exemplo_array() {
    let funcionarios: [(i32, String, f32); 2] = [
        (42, String::from("david"), 30000.0),
        (43, String::from("maria"), 35000.0),
    ];
    println!("==========Exemplo de array:==========");
    for (id, nome, salario) in &funcionarios {
        println!("ID: {}, Nome: {}, Salário: {}", id, nome, salario);
    }
}

// Exemplo de struct
#[derive(Debug)]
struct Funcionario {
    id: i32,
    nome: String,
    salario: f32,
}

fn exemplo_struct() {
    let funcionario = Funcionario {
        id: 42,
        nome: String::from("david"),
        salario: 30000.0,
    };

    println!("==========Exemplo de struct:==========");
    println!("Funcionário: {:?}", funcionario);
    println!("ID: {}", funcionario.id);
    println!("Nome: {}", funcionario.nome);
    println!("Salário: {}", funcionario.salario);
}

// Exemplo de vetor (Vec) de structs
fn exemplo_vec() {
    let funcionarios = vec![
        Funcionario { id: 42, nome: String::from("david"), salario: 30000.0 },
        Funcionario { id: 43, nome: String::from("maria"), salario: 35000.0 },
        Funcionario { id: 44, nome: String::from("joão"), salario: 40000.0 },
    ];

    println!("==========Exemplo de vetor (Vec):==========");
    for f in &funcionarios {
        println!("ID: {}, Nome: {}, Salário: {}", f.id, f.nome, f.salario);
    }
}

fn main() {
    exemplo_tupla();
    exemplo_array();
    exemplo_struct();
    exemplo_vec();
}
