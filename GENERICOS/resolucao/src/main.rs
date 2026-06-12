use std::fmt::Display;
use serde::Serialize;
use serde_json::to_string_pretty;

// ===== Exemplo 1: Função genérica para contar posições =====
fn contar_posicoes<T>(array: &[T]) -> usize {
    array.len()
}

// ===== Exemplo 2: Trait para contar caracteres =====
trait ContaCaracteres {
    fn conta_caracteres(&self) -> usize;
}

impl ContaCaracteres for i32 {
    fn conta_caracteres(&self) -> usize {
        self.to_string().chars().count()
    }
}

impl ContaCaracteres for f64 {
    fn conta_caracteres(&self) -> usize {
        self.to_string().chars().count()
    }
}

impl ContaCaracteres for String {
    fn conta_caracteres(&self) -> usize {
        self.chars().count()
    }
}

impl<'a> ContaCaracteres for &'a str {
    fn conta_caracteres(&self) -> usize {
        self.chars().count()
    }
}

fn quantidade_caracteres<T: ContaCaracteres>(valor: T) -> usize {
    valor.conta_caracteres()
}

// ===== Exemplo 3: Usando Display para simplificar =====
fn quantidade_caracteres_display<T: Display>(valor: T) -> usize {
    valor.to_string().chars().count()
}

// ===== Exemplo 4: Struct genérica =====
struct Point<T> {
    x: T,
    _y: T, // prefixado para evitar warning
}

impl<T> Point<T> {
    fn retorna_valor_de_x(&self) -> &T {
        &self.x
    }
}

// ===== Exemplo 5: Struct genérica com dois tipos =====
struct Pair<T, U> {
    _x: T, // prefixado para evitar warning
    y: U,
}

impl<T, U> Pair<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { _x: x, y }
    }
}

// ===== Exemplo 6: Função genérica com trait bounds =====
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ===== Exemplo 7: Função genérica para imprimir propriedades =====
fn imprimir_propriedades(item: &impl Serialize) {
    let json = to_string_pretty(item).unwrap_or_else(|_| "Falha na serialização".to_string());
    println!("{}", json);
}

#[derive(Serialize)]
struct Produto {
    id: u32,
    nome: String,
    preco: f64,
}

#[derive(Serialize)]
struct Cliente {
    id: u32,
    nome: String,
    email: String,
}

// ===== MAIN =====
fn main() {
    // Exemplo 1
    let array_inteiros = [1, 2, 3, 4, 5];
    println!("Posições no array de inteiros: {}", contar_posicoes(&array_inteiros));

    // Exemplo 2
    let int_val = 12345;
    let float_val = 123.45;
    let str_val = "Olá josé".to_string();
    println!("Quantidade de caracteres no inteiro: {}", quantidade_caracteres(int_val));
    println!("Quantidade de caracteres no float: {}", quantidade_caracteres(float_val));
    println!("Quantidade de caracteres na string: {}", quantidade_caracteres(str_val));

    // Exemplo 3
    println!("Usando Display: {}", quantidade_caracteres_display(9876));

    // Exemplo 4
    let p = Point { x: 5, _y: 10 };
    println!("p.x = {}", p.retorna_valor_de_x());

    // Exemplo 5
    let pair = Pair::new(5, "Texto genérico");
    println!("Pair.y = {}", pair.y);

    // Exemplo 6
    let numbers = vec![34, 50, 25, 100, 65];
    println!("O maior número é {}", largest(&numbers));

    // Exemplo 7
    let produto = Produto { id: 1, nome: "Caneta".to_string(), preco: 1.50 };
    let cliente = Cliente { id: 101, nome: "João Silva".to_string(), email: "joao.silva@example.com".to_string() };
    imprimir_propriedades(&produto);
    imprimir_propriedades(&cliente);
}
