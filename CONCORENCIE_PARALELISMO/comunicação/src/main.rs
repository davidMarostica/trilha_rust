
use std::sync::mpsc; // mpsc significa multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    // Cria um canal
    let (tx, rx) = mpsc::channel(); // tx = envia o resultado, rx = recebe o resultado

    // Cria uma nova thread
    thread::spawn(move || {
        let msg = String::from("Olá, Pessoal!");

        thread::sleep(Duration::from_millis(1000));

        // Envia uma mensagem pelo canal
        tx.send(msg).unwrap();
        // Note que `msg` não pode mais ser usado aqui, pois `send` toma a propriedade da mensagem
    });


    // Recebe a mensagem na thread principal
    let received = rx.recv().unwrap(); // espera até a thread ser resolvida
    println!("Mensagem recebida: {}", received);
}
