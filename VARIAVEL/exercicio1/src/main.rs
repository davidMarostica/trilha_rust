fn main() {
    /* dado que eue tenha um ano de nascimento, e faço a subitração pelo ano atual,
    Então devo ter o valor da idade da pessoa
    */
    let ano_nascimento: i32 = 1983;
    let mes_nascimento: i32 = 6;
    let dia_nascimento: i32 = 14;

    let ano_atual: i32 = 2026;
    let mes_atual: i32 = 5;
    let dia_atual: i32 = 15;

    let mut idade: i32 = ano_atual - ano_nascimento;
    if mes_atual < mes_nascimento {
        println!("A pessoa ainda não fez aniversário este ano.");
        idade -= 1;
    } else if mes_atual == mes_nascimento && dia_atual < dia_nascimento {
        println!("A pessoa ainda não fez aniversário este ano.");
        idade -= 1;
    } else {
        println!("A pessoa já fez aniversário este ano.");
    }

    println!("A idade da pessoa é: {}", idade);
}
