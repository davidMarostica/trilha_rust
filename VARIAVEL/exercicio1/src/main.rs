fn main() {
    /* dado que eue tenha um ano de nascimento, e faço a subitração pelo ano atual,
    Então devo ter o valor da idade da pessoa
    */
    let ano_nascimento: i32 = 1983;
    let ano_atual: i32 = 2026;
    let idade: i32 = ano_atual - ano_nascimento;

    println!("A idade da pessoa é: {}", idade);
}
