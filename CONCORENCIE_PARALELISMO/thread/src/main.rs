use std::thread;
use std::time::Duration;

fn main() {

    // for i in 1..=5 {
    //     println!("{}", i);
    //     thread::sleep(Duration::from_millis(500));
    // }

    // for letter in 'a'..='e' {
    //     println!("{}", letter);
    //     thread::sleep(Duration::from_millis(200));
    // }


    // Thread para imprimir números
    let num_thread = thread::spawn(|| {
        for i in 1..=5 {
            println!("{}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Thread para imprimir letras
    let letter_thread = thread::spawn(|| {
        for letter in 'a'..='e' {
            println!("{}", letter);
            thread::sleep(Duration::from_millis(200));
        }
    });

    // Espera as threads terminarem
    num_thread.join().unwrap();
    letter_thread.join().unwrap();
}
