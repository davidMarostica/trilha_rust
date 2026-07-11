/////// ======= Macro declarativa simples =======

macro_rules! diz_ola {
    () => {
        println!("Olá, mundo!")
    };
}

fn main() {
    diz_ola!()
}