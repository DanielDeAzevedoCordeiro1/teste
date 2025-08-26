use std::fmt::format;

struct Pessoa {
    name: String,
    age: u8
}

impl Pessoa{
    fn new(name: String, age: u8) -> Pessoa{
        Pessoa{name,age}
    }

    fn to_string(&self){
        println!("Nome: {}   Idade: {}",self.name,self.age)
    }
}


trait PessoaTrait {
    fn new(name: String, age: u8) -> Self;
    fn to_string(&self);
}

trait Dancar {
    fn dancar(&self) -> String;
}

trait Correr {
    fn correr(&self) -> String;
}

impl PessoaTrait for Pessoa {
    fn new(name: String, age: u8) -> Self {
        Pessoa::new(name, age)
    }

    fn to_string(&self) {
        self.to_string();
    }
}

trait Cantar {
    fn cantar(&self, musica: &str) -> String;
}

trait Saltar {
    fn saltar(&self) -> String;
}

trait  LancarEvento {
    fn lancar_evento(&self, evento: &str) -> String;
}

trait ToString {
    fn to_string(&self);
    
}

impl Cantar for Pessoa {
    fn cantar(&self, musica: &str) -> String {
        return format!("{} esta cantando {}", self.name, musica);
    }
}

trait Pagar {
    fn pagar(&self) -> String;
}

trait FazerBarulho {
    fn fazer_barulho(&self) -> String;
}

trait FazerCommit {
    fn fazer_commit(&self, mensagem: &str, branch: &str) -> String;

}

trait SolicitarCredito{
    fn solicitar_credito(&self, valor: f64, banco: &str) -> String;
}

trait DebitarFatura {
    fn debitar_fatura(&self, valor: f64, cartao: &str) -> String;
}

trait SolicitarLimiteCartao {
    fn solicitar_limite_cartao(&self, valor: f64, cartao: &str) -> String;
}


trait EditarCommit {
    fn editar_commit(&self, mensagem: &str, branch: &str) -> String;
}

trait AbrirDireitorio {
    fn abrir_diretorio(&self) -> String;
    
}

trait EditarMensagemCommit {
    fn editar_mensagem_commit(&self, mensagem: &str, branch: &str) -> String;
}

trait CriarBranch {
    fn criar_branch(&self, nome: &str) -> String;
}

trait Rebase{
    fn rebase(&self, branch: &str, branch_base: &str) -> String;
}

trait criarPasta {
    fn criar_pasta(&self, nome: &str) -> String;
}

trait Jogar {
    fn jogar(&self, jogo: &str) -> String;
}
trait DeletarConta {
    fn deletar_conta(&self, conta: &str, banco: &str) -> String;
}

trait Depositar {
    fn depositar(&self, valor: f64, conta: &str, banco: &str) -> String;
}

trait EditarNomeDiretorio {
    fn editar_nome_diretorio(&self, nome_antigo: &str, nome_novo: &str) -> String;
}

trait FazerPedido {
    fn fazer_pedido(&self, pedido: &str) -> String;
}

trait CriarConGoogleDrive {
    fn criar_con_google_drive(&self, nome: &str) -> String;
}

trait DeletarDiretorio {
    fn deletar_diretorio(&self, nome: &str) -> String;
}
trait CherryPick {
    fn cherry_pick(&self, commit: &str, branch: &str) -> String;
    
}
trait Merge {
    fn merge(&self, branch: &str, branch_base: &str) -> String;
}

trait Amend {
    fn amend(&self, mensagem: &str, branch: &str) -> String;
}

trait TrocarBranch {
    fn trocar_branch(&self, branch: &str, branch_nova: &str) -> String;
}

trait CalcularFrete{
    fn calcular_frete(&self, peso: f64, distancia: f64) -> String;
}

impl SolicitarCredito for Pessoa {
    fn solicitar_credito(&self, valor: f64, banco: &str) -> String {
        return format!("{} esta solicitando um credito de {} no banco {}", self.name, valor, banco);
    }
}

impl DebitarFatura for Pessoa {
    fn debitar_fatura(&self, valor: f64, cartao: &str) -> String {
        return format!("{} esta debitando {} na fatura do cartao {}", self.name, valor, cartao);
    }
}

impl SolicitarLimiteCartao for Pessoa {
    fn solicitar_limite_cartao(&self, valor: f64, cartao: &str) -> String {
        return format!("{} esta solicitando um limite de {} no cartao {}", self.name, valor, cartao);
    }
}

impl Amend for Pessoa {
    fn amend(&self, mensagem: &str, branch: &str) -> String {
        return format!("{} esta fazendo amend na branch {} com a mensagem: {}", self.name, branch, mensagem);
    }
    
}

impl Dancar for Pessoa {
    fn dancar(&self) -> String {
        return format!("{} esta dancando", self.name);
    }
}

impl EditarMensagemCommit for Pessoa {
    fn editar_mensagem_commit(&self, mensagem: &str, branch: &str) -> String {
        return format!("{} esta editando a mensagem do commit na branch {} com a mensagem: {}", self.name, branch, mensagem);
    }
    
}

impl CriarBranch for Pessoa {
    fn criar_branch(&self, nome: &str) -> String {
        return format!("{} esta criando a branch {}", self.name, nome);
    }
}

impl Rebase for Pessoa {
    fn rebase(&self, branch: &str, branch_base: &str) -> String {
        return format!("{} esta fazendo rebase da branch {} com a branch base {}", self.name, branch, branch_base);
    }
}

impl FazerCommit for Pessoa {
    fn fazer_commit(&self, mensagem: &str, branch: &str) -> String {
        return format!("{} esta fazendo commit na branch {} com a mensagem: {}", self.name, branch, mensagem);
    }
    
}

impl CalcularFrete for Pessoa {
    fn calcular_frete(&self, peso: f64, distancia: f64) -> String {
        let valor_frete = peso * distancia * 0.1; // Exemplo de cálculo simples
        return format!("{} esta calculando o frete: R${:.2}", self.name, valor_frete);
    }
}

impl TrocarBranch for Pessoa {
    fn trocar_branch(&self, branch: &str, branch_nova: &str) -> String {
        return format!("{} esta trocando da branch {} para a branch {}", self.name, branch, branch_nova);
    }
    
}

impl EditarCommit for Pessoa {
    fn editar_commit(&self, mensagem: &str, branch: &str) -> String {
        return format!("{} esta editando o commit na branch {} com a mensagem: {}", self.name, branch, mensagem);
    }
}

impl LancarEvento for Pessoa {
    fn lancar_evento(&self, evento: &str) -> String {
        return format!("{} esta lancando o evento {}", self.name, evento);
    }
}

impl DeletarConta for Pessoa {
    fn deletar_conta(&self, conta: &str, banco: &str) -> String {
        return format!("{} esta deletando a conta {} do banco {}", self.name, conta, banco);
    }
    
}

impl CriarConGoogleDrive for Pessoa {
    fn criar_con_google_drive(&self, nome: &str) -> String {
        return format!("{} esta criando uma conexao com o Google Drive chamada {}", self.name, nome);
    }
}

impl Depositar for Pessoa {
    fn depositar(&self, valor: f64, conta: &str, banco: &str) -> String {
        return format!("{} esta depositando {} na conta {} do banco {}", self.name, valor, conta, banco);
    }
    
}

impl CherryPick for Pessoa {
    fn cherry_pick(&self, commit: &str, branch: &str) -> String {
        return format!("{} esta fazendo cherry-pick do commit {} na branch {}", self.name, commit, branch);
    }
    
}

impl EditarNomeDiretorio for Pessoa {
    fn editar_nome_diretorio(&self, nome_antigo: &str, nome_novo: &str) -> String {
        return format!("{} esta editando o nome do diretorio de {} para {}", self.name, nome_antigo, nome_novo);
    }
}

impl Jogar for Pessoa {
    fn jogar(&self, jogo: &str) -> String {
        return format!("{} esta jogando {}", self.name, jogo);
    }
}

impl criarPasta for Pessoa {
    fn criar_pasta(&self, nome: &str) -> String {
        return format!("{} esta criando a pasta {}", self.name, nome);
    }
    
}

impl Merge for Pessoa {
    fn merge(&self, branch: &str, branch_base: &str) -> String {
        return format!("{} esta fazendo merge da branch {} com a branch base {}", self.name, branch, branch_base);
    }
}

impl DeletarDiretorio for Pessoa {
    fn deletar_diretorio(&self, nome: &str) -> String {
        return format!("{} esta deletando o diretorio {}", self.name, nome);
    }
    
}

impl Pagar for Pessoa {
    fn pagar(&self) -> String {
        return format!("{} esta pagando", self.name);
    }
    
}

impl FazerPedido for Pessoa {
    fn fazer_pedido(&self, pedido: &str) -> String {
        return format!("{} esta fazendo o pedido de {}", self.name, pedido);
    }
}

impl FazerBarulho for Pessoa {
    fn fazer_barulho(&self) -> String {
        return format!("{} esta fazendo barulho", self.name);
    }
}

impl AbrirDireitorio for Pessoa {
    fn abrir_diretorio(&self) -> String {
        return format!("{} esta abrindo o diretorio", self.name);
    }
}

impl Saltar for Pessoa {
    fn saltar(&self) -> String {
        return format!("{} esta saltando", self.name);
    }
}

impl ToString for Pessoa {
    fn to_string(&self) {
        println!("Nome: {}   Idade: {}", self.name, self.age);
    }
}

impl Correr for Pessoa {
    fn correr(&self) -> String {
        return format!("{} esta correndo", self.name);
    }
}

mod structs{
    pub mod banco_structs;
}


use  structs::banco_structs::*;
fn main(){


    let pessoa = Pessoa::new("Daniel".to_string(), 23);
    pessoa.to_string();
    print!("{}",pessoa.dancar());
    print!("{}",pessoa.correr());
    print!("{}",pessoa.saltar());
    print!("{}",pessoa.fazer_barulho());
    print!("{}",pessoa.abrir_diretorio());
    println!();
    let pessoa2 = Pessoa::new("Maria".to_string(), 30);
    pessoa2.to_string();
    print!("{}",pessoa2.pagar());
    print!("{}",pessoa2.jogar("Futebol"));
    print!("{}",pessoa2.fazer_pedido("Pizza"));
    print!("{}",pessoa2.cantar("Despacito"));
    print!("{}",pessoa2.depositar(1000.0, "12345", "Banco do Brasil"));
    print!("{}",pessoa2.deletar_conta("12345", "Banco do Brasil"));
    print!("{}",pessoa2.criar_pasta("Documentos"));
    print!("{}",pessoa2.deletar_diretorio("Downloads"));
    print!("{}",pessoa2.editar_nome_diretorio("Antigo", "Novo"));
    print!("{}",pessoa2.criar_con_google_drive("Minha Conexao"));
    print!("{}",pessoa2.lancar_evento("Festa"));
    print!("{}",pessoa2.fazer_commit("Adicionando novas funcionalidades", "main"));
    print!("{}",pessoa2.editar_commit("Corrigindo bugs", "main"));
    print!("{}",pessoa2.merge("feature", "main"));
    print!("{}",pessoa2.rebase("feature", "main"));
    print!("{}",pessoa2.amend("Atualizando commit", "main"));
    print!("{}",pessoa2.cherry_pick("abc123", "main"));
    print!("{}",pessoa2.criar_branch("nova_branch"));
    print!("{}",pessoa2.trocar_branch("main", "nova_branch"));
    print!("{}",pessoa2.editar_mensagem_commit("Atualizando mensagem do commit", "main"));
    print!("{}",pessoa2.solicitar_credito(5000.0, "Banco do Brasil"));
    print!("{}",pessoa2.debitar_fatura(200.0, "Cartao de Credito"));
    print!("{}",pessoa2.solicitar_limite_cartao(3000.0, "Cartao de Credito"));
    print!("{}",pessoa2.calcular_frete(10.0, 100.0));

    let mut conta_simples = ContaSimples{
        titular: "João".to_string(),
        saldo: 10.0,
    };
    
    conta_simples.depositar(50.0);
    conta_simples.consultar_saldo();


    let mut corrente1 = ContaCorrente {
        titular: "Ana".to_string(),
        saldo: 200.0,
        limite: 100.0,
        numero: "001".to_string(),
    };
    let mut corrente2 = ContaCorrente {
        titular: "Carlos".to_string(),
        saldo: 50.0,
        limite: 50.0,
        numero: "002".to_string(),
    };
    corrente1.transferir(100.0, &mut corrente2);
    println!("Saldo Ana: {}", corrente1.consultar_saldo());
    println!("Saldo Carlos: {}", corrente2.consultar_saldo());


    let mut investimento = ContaInvestimento {
        titular: "Beatriz".to_string(),
        saldo: 1000.0,
        carteira: vec![],
        rendimento_anual: 0.08,
        risco: "Moderado".to_string(),
    };
    investimento.investir("Tesouro Direto".to_string(), 500.0);
    println!("Saldo Investimento: {}", investimento.consultar_saldo());
    println!("Rendimento anual: {:.2}", investimento.calcular_rendimento());
    println!("Risco: {}", investimento.avaliar_risco());

    let mut empresa = ContaEmpresarial {
        titular: "Tech Ltda".to_string(),
        saldo: 5000.0,
        cnpj: "12.345.678/0001-99".to_string(),
        funcionarios: 10,
    };

    empresa.depositar(2000.0);
    println!("Saldo ContaEmpresarial: {}", empresa.consultar_saldo());

    let pagou = empresa.pagar_funcionarios(3000.0);
    println!("Pagou funcionários? {}", pagou);
    println!("Saldo após pagamento: {}", empresa.consultar_saldo());


      let mut poupanca = ContaPoupanca {
        titular: "Lucas".to_string(),
        saldo: 500.0,
        rendimento_mensal: 0.005, 
    };

    poupanca.depositar(200.0);
    println!("Saldo ContaPoupanca: {}", poupanca.consultar_saldo());

    let sacou = poupanca.sacar(100.0);
    println!("Sacou da poupança? {}", sacou);
    println!("Saldo após saque: {}", poupanca.consultar_saldo());

    let simulado = poupanca.simular_rendimento(6);
    println!("Saldo simulado após 6 meses: {:.2}", simulado);

     let mut digital = ContaDigital {
        titular: "Sofia".to_string(),
        saldo: 800.0,
        pix_key: "sofia@email.com".to_string(),
        cashback_rate: 0.02, 
    };

    digital.depositar(300.0);
    println!("Saldo ContaDigital: {}", digital.consultar_saldo());

    let pix_enviado = digital.pix(150.0, "maria@email.com");
    println!("PIX enviado? {}", pix_enviado);
    println!("Saldo após PIX: {}", digital.consultar_saldo());

    let cashback = digital.calcular_cashback(500.0);
    println!("Cashback em compra de R$500: R${:.2}", cashback);

     let mut juridica1 = ContaJuridica {
        titular: "Empresa XYZ Ltda".to_string(),
        saldo: 10000.0,
        cnpj: "11.222.333/0001-44".to_string(),
        taxa_movimentacao: 0.01, // 1% de taxa
        limite_diario: 5000.0,
    };

    let mut juridica2 = ContaJuridica {
        titular: "Startup ABC".to_string(),
        saldo: 2000.0,
        cnpj: "22.333.444/0001-55".to_string(),
        taxa_movimentacao: 0.015, // 1.5% de taxa
        limite_diario: 3000.0,
    };

    juridica1.depositar(5000.0);
    println!("Saldo Conta Jurídica 1: {}", juridica1.consultar_saldo());

    let transferiu = juridica1.transferir_com_taxa(2000.0, &mut juridica2);
    println!("Transferência realizada? {}", transferiu);
    println!("Saldo após transferência - Empresa XYZ: {}", juridica1.consultar_saldo());
    println!("Saldo após transferência - Startup ABC: {}", juridica2.consultar_saldo());

    println!("{}", juridica1.gerar_extrato());
    println!("{}", juridica2.gerar_extrato());
}
