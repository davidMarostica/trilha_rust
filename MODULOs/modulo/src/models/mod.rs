use crate ::enums ::Tipo;

pub struct Pessoa {
    pub nome: String,
    pub documento: String,
    pub tipo: Tipo,
}

impl Pessoa {
    pub fn new(nome: &str, documento: &str, tipo: Tipo) -> Self {
        Pessoa { nome: nome.to_string(), 
            documento: documento.to_string(), 
            tipo: tipo }
    }


    pub fn show(&self) {
        println!("Nome: {}", self.nome);
        println!("Documento: {}", self.documento);
        match self.tipo {
            Tipo::Juridica => println!("Tipo: Jurídica"),
            Tipo::Fisica => println!("Tipo: Física"),
        }
    }


    pub fn tipo_string(&self) -> &str {
        match self.tipo {
            Tipo::Juridica => "Jurídica",
            Tipo::Fisica => "Física",
        }
    }
}