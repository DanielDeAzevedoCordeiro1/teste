
pub struct ContaSimples {
    pub titular: String,
    pub saldo: f64,
}


pub struct ContaCorrente {
    pub titular: String,
    pub saldo: f64,
    pub limite: f64,
    pub numero: String,
}

pub struct ContaInvestimento {
    pub titular: String,
    pub saldo: f64,
    pub carteira: Vec<String>,
    pub rendimento_anual: f64,
    pub risco: String,
}