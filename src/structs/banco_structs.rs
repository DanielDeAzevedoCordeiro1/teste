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

pub trait OperacoesSimples {
    fn depositar(&mut self, valor: f64);
    fn sacar(&mut self, valor: f64) -> bool;
    fn consultar_saldo(&self) -> f64;
}

pub trait OperacoesCorrente {
    fn depositar(&mut self, valor: f64);
    fn sacar(&mut self, valor: f64) -> bool;
    fn consultar_saldo(&self) -> f64;
    fn usar_limite(&mut self, valor: f64) -> bool;
    fn transferir(&mut self, valor: f64, destino: &mut ContaCorrente) -> bool;
}

pub trait OperacoesInvestimento {
    fn consultar_saldo(&self) -> f64;
    fn investir(&mut self, ativo: String, valor: f64) -> bool;
    fn calcular_rendimento(&self) -> f64;
    fn avaliar_risco(&self) -> &str;
}