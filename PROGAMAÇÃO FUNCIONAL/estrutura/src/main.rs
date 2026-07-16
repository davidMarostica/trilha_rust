// === Abordagem com struct (Com abordagem em Orientação a Objetos, pois usa conceito de mutabilidade) =======

struct CalculadoraSalario {
    salario_bruto: f64,
    total_descontos: f64,
}

impl CalculadoraSalario {
    fn new(salario_bruto: f64) -> Self {
        CalculadoraSalario {
            salario_bruto: salario_bruto,
            total_descontos: 0.0,
        }
    }

    fn desconto_plano_saude(mut self) -> Self {
        self.total_descontos += self.salario_bruto * 0.10; // Desconto de 10%
        self
    }

    fn desconto_plano_dentario(mut self) -> Self {
        self.total_descontos += self.salario_bruto * 0.05; // Desconto de 5%
        self
    }

    fn desconto_vale_refeicao(mut self) -> Self {
        self.total_descontos += self.salario_bruto * 0.03; // Desconto de 3%
        self
    }

    fn valor(self) -> f64 {
        self.salario_bruto - self.total_descontos
    }
}

fn main() {
    let salario_bruto = 10000.0;
    let salario_liquido = CalculadoraSalario::new(salario_bruto)
        .desconto_plano_saude()
        .desconto_plano_dentario()
        .desconto_vale_refeicao()
        .valor();

    println!("Salário líquido: {:.2}", salario_liquido);
}