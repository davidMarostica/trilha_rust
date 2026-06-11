#[derive(Debug)]
pub enum Sexo {
    Masculino,
    Feminino,
    Outros,
}

impl Sexo {
    pub fn descricao(&self) -> &str {
        match self {
            Sexo::Masculino => "Masculino",
            Sexo::Feminino => "Feminino",
            Sexo::Outros => "Outros",
        }
    }
}
