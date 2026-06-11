#[derive(Debug)]
pub enum Tipo {
    Juridica,
    Fisica,
}

impl Tipo {
    pub fn descricao(&self) -> &str {
        match self {
            Tipo::Juridica => "Jurídica",
            Tipo::Fisica => "Física",
        }
    }
}
