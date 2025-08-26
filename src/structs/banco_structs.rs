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

pub struct ContaEmpresarial {
    pub titular: String,
    pub saldo: f64,
    pub cnpj: String,
    pub funcionarios: u32,
}

pub struct ContaPoupanca {
    pub titular: String,
    pub saldo: f64,
    pub rendimento_mensal: f64,
}

pub struct ContaDigital {
    pub titular: String,
    pub saldo: f64,
    pub pix_key: String,
    pub cashback_rate: f64,
}

pub struct ContaJuridica {
    pub titular: String,
    pub saldo: f64,
    pub cnpj: String,
    pub taxa_movimentacao: f64,
    pub limite_diario: f64,
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


pub trait OperacoesEmpresariais {
    fn depositar(&mut self, valor: f64);
    fn pagar_funcionarios(&mut self, valor_total: f64) -> bool;
    fn consultar_saldo(&self) -> f64;
}

pub trait OperacoesPoupanca {
    fn depositar(&mut self, valor: f64);
    fn sacar(&mut self, valor: f64) -> bool;
    fn consultar_saldo(&self) -> f64;
    fn simular_rendimento(&self, meses: u32) -> f64;
}

pub trait OperacoesDigitais {
    fn depositar(&mut self, valor: f64);
    fn sacar(&mut self, valor: f64) -> bool;
    fn consultar_saldo(&self) -> f64;
    fn pix(&mut self, valor: f64, chave_destino: &str) -> bool;
    fn calcular_cashback(&self, valor_compra: f64) -> f64;
}

pub trait OperacoesJuridicas {
    fn depositar(&mut self, valor: f64);
    fn sacar(&mut self, valor: f64) -> bool;
    fn consultar_saldo(&self) -> f64;
    fn transferir_com_taxa(&mut self, valor: f64, destino: &mut ContaJuridica) -> bool;
    fn verificar_limite_diario(&self, valor: f64) -> bool;
    fn gerar_extrato(&self) -> String;
}

impl OperacoesSimples for ContaSimples {
    fn depositar(&mut self, valor: f64) {
        self.saldo += valor;
    }

    fn sacar(&mut self, valor: f64) -> bool {
        if self.saldo >= valor {
            self.saldo -= valor;
            true
        } else {
            false
        }
    }

    fn consultar_saldo(&self) -> f64 {
        self.saldo
    }
}

impl OperacoesCorrente for ContaCorrente {
    fn depositar(&mut self, valor: f64) {
        self.saldo += valor;
    }

    fn sacar(&mut self, valor: f64) -> bool {
        if self.saldo + self.limite >= valor {
            self.saldo -= valor;
            true
        } else {
            false
        }
    }

    fn consultar_saldo(&self) -> f64 {
        self.saldo
    }

    fn usar_limite(&mut self, valor: f64) -> bool {
        if self.limite >= valor {
            self.limite -= valor;
            self.saldo += valor;
            true
        } else {
            false
        }
    }

    fn transferir(&mut self, valor: f64, destino: &mut ContaCorrente) -> bool {
        if self.sacar(valor) {
            destino.depositar(valor);
            true
        } else {
            false
        }
    }
}

impl OperacoesInvestimento for ContaInvestimento {
    fn consultar_saldo(&self) -> f64 {
        self.saldo
    }

    fn investir(&mut self, ativo: String, valor: f64) -> bool {
        if self.saldo >= valor {
            self.saldo -= valor;
            self.carteira.push(ativo);
            true
        } else {
            false
        }
    }

    fn calcular_rendimento(&self) -> f64 {
        self.saldo * self.rendimento_anual
    }

    fn avaliar_risco(&self) -> &str {
        &self.risco
    }
}

impl OperacoesEmpresariais for ContaEmpresarial {
    fn depositar(&mut self, valor: f64) {
        self.saldo += valor;
    }

    fn pagar_funcionarios(&mut self, valor_total: f64) -> bool {
        if self.saldo >= valor_total {
            self.saldo -= valor_total;
            true
        } else {
            false
        }
    }

    fn consultar_saldo(&self) -> f64 {
        self.saldo
    }
}

impl OperacoesPoupanca for ContaPoupanca {
    fn depositar(&mut self, valor: f64) {
        self.saldo += valor;
    }

    fn sacar(&mut self, valor: f64) -> bool {
        if self.saldo >= valor {
            self.saldo -= valor;
            true
        } else {
            false
        }
    }

    fn consultar_saldo(&self) -> f64 {
        self.saldo
    }

    fn simular_rendimento(&self, meses: u32) -> f64 {
        let mut saldo = self.saldo;
        for _ in 0..meses {
            saldo += saldo * self.rendimento_mensal;
        }
        saldo
    }
}

impl OperacoesDigitais for ContaDigital {
    fn depositar(&mut self, valor: f64) {
        self.saldo += valor;
    }

    fn sacar(&mut self, valor: f64) -> bool {
        if self.saldo >= valor {
            self.saldo -= valor;
            true
        } else {
            false
        }
    }

    fn consultar_saldo(&self) -> f64 {
        self.saldo
    }

    fn pix(&mut self, valor: f64, chave_destino: &str) -> bool {
        if self.saldo >= valor {
            self.saldo -= valor;
            println!("PIX de R${:.2} enviado para {}", valor, chave_destino);
            true
        } else {
            false
        }
    }

    fn calcular_cashback(&self, valor_compra: f64) -> f64 {
        valor_compra * self.cashback_rate
    }
}

impl OperacoesJuridicas for ContaJuridica {
    fn depositar(&mut self, valor: f64) {
        self.saldo += valor;
    }

    fn sacar(&mut self, valor: f64) -> bool {
        let valor_com_taxa = valor + (valor * self.taxa_movimentacao);
        if self.saldo >= valor_com_taxa && self.verificar_limite_diario(valor) {
            self.saldo -= valor_com_taxa;
            true
        } else {
            false
        }
    }

    fn consultar_saldo(&self) -> f64 {
        self.saldo
    }

    fn transferir_com_taxa(&mut self, valor: f64, destino: &mut ContaJuridica) -> bool {
        if self.sacar(valor) {
            destino.depositar(valor);
            true
        } else {
            false
        }
    }

    fn verificar_limite_diario(&self, valor: f64) -> bool {
        valor <= self.limite_diario
    }

    fn gerar_extrato(&self) -> String {
        format!("Extrato - Empresa: {} | CNPJ: {} | Saldo: R${:.2}", 
                self.titular, self.cnpj, self.saldo)
    }
}