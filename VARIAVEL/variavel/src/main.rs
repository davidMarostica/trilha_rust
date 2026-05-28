use std::sync::atomic::{AtomicI8, Ordering};

const TIPO_DE_DADO: i8 = 2;
static UMA_VARIAVEL_STATICA: AtomicI8 = AtomicI8::new(3);

fn main() {
    UMA_VARIAVEL_STATICA.store(4, Ordering::SeqCst);
    imprime();
}

fn imprime() {
    println!("constante: {}", TIPO_DE_DADO);
    println!("variável estática: {}", UMA_VARIAVEL_STATICA.load(Ordering::SeqCst));
}
