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

trait AbrirDireitorio {
    fn abrir_diretorio(&self) -> String;
    
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

trait FazerPedido {
    fn fazer_pedido(&self, pedido: &str) -> String;
}


impl Dancar for Pessoa {
    fn dancar(&self) -> String {
        return format!("{} esta dancando", self.name);
    }
}

impl DeletarConta for Pessoa {
    fn deletar_conta(&self, conta: &str, banco: &str) -> String {
        return format!("{} esta deletando a conta {} do banco {}", self.name, conta, banco);
    }
    
}

impl Depositar for Pessoa {
    fn depositar(&self, valor: f64, conta: &str, banco: &str) -> String {
        return format!("{} esta depositando {} na conta {} do banco {}", self.name, valor, conta, banco);
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
}
