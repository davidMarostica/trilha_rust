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

// Struct principal
#[derive(Debug)]
struct Funcionario {
    id: i32,
    nome: String,
    salario: f32,
}

impl Funcionario {
    // Construtor "null"
    fn null() -> Funcionario {
        Funcionario {
            id: 0,
            nome: String::from(""),
            salario: 0.0,
        }
    }

    // Construtor "build"
    fn build(id: i32, nome: &str, salario: f32) -> Funcionario {
        Funcionario {
            id,
            nome: String::from(nome),
            salario,
        }
    }
}

// Struct com endereço
#[derive(Debug)]
struct Endereco {
    rua: String,
    numero: i32,
    cidade: String,
}

// Struct composta (Funcionario + Endereco)
#[derive(Debug)]
struct FuncionarioCompleto {
    funcionario: Funcionario,
    endereco: Endereco,
}

// Struct com valores opcionais
#[derive(Debug)]
struct FuncionarioNull {
    id: Option<i32>,
    nome: Option<String>,
    salario: Option<f32>,
}

fn exemplo_structs() {
    let funcionario = Funcionario::build(42, "david", 30000.0);
    let funcionario_null = Funcionario::null();

    let endereco = Endereco {
        rua: String::from("Rua das Flores"),
        numero: 123,
        cidade: String::from("Jundiaí"),
    };

    let funcionario_completo = FuncionarioCompleto {
        funcionario,
        endereco,
    };

    let funcionario_opcional = FuncionarioNull {
        id: None,
        nome: None,
        salario: None,
    };

    println!("==========Exemplo de struct com build:==========");
    println!("{:?}", funcionario_completo.funcionario);

    println!("==========Exemplo de struct null:==========");
    println!("{:?}", funcionario_null);

    println!("==========Exemplo de struct com endereço:==========");
    println!("{:?}", funcionario_completo.endereco);

    println!("==========Exemplo de struct composta:==========");
    println!("{:?}", funcionario_completo);

    println!("==========Exemplo de struct com Option:==========");
    println!("{:?}", funcionario_opcional);
}

// Exemplo de vetor (Vec) de structs
fn exemplo_vec() {
    let funcionarios = vec![
        Funcionario::build(42, "david", 30000.0),
        Funcionario::build(43, "maria", 35000.0),
        Funcionario::build(44, "joão", 40000.0),
    ];

    println!("==========Exemplo de vetor (Vec):==========");
    for f in &funcionarios {
        println!("ID: {}, Nome: {}, Salário: {}", f.id, f.nome, f.salario);
    }
}

fn main() {
    exemplo_tupla();
    exemplo_array();
    exemplo_structs();
    exemplo_vec();
}
