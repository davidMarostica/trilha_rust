use crate::enums::tipo::Tipo;
use crate::enums::sexo::Sexo;

#[derive(Debug)]
pub struct Pessoa {
    pub nome: String,
    pub documento: String,
    pub tipo: Tipo,
    pub sexo: Sexo,
}

impl Pessoa {
    pub fn new(nome: &str, documento: &str, tipo: Tipo, sexo: Sexo) -> Self {
        Pessoa {
            nome: nome.to_string(),
            documento: documento.to_string(),
            tipo,
            sexo,
        }
    }

    pub fn show(&self) {
        println!("Nome: {}", self.nome);
        println!("Documento: {}", self.documento);
        println!("Tipo: {}", self.tipo.descricao());
        println!("Sexo: {}", self.sexo.descricao());
    }
}
