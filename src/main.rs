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

trait Dancar {
    fn dancar(&self) -> String;
}

trait Correr {
    fn correr(&self) -> String;
}

trait Saltar {
    fn saltar(&self) -> String;
}

trait ToString {
    fn to_string(&self);
    
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

trait Jogar {
    fn jogar(&self, jogo: &str) -> String;
}

trait Trabalhar {
    fn trabalhar(&self, horas: u32) -> String;
}

impl Dancar for Pessoa {
    fn dancar(&self) -> String {
        return format!("{} esta dancando", self.name);
    }
}

impl Jogar for Pessoa {
    fn jogar(&self, jogo: &str) -> String {
        return format!("{} esta jogando {}", self.name, jogo);
    }
}

impl Pagar for Pessoa {
    fn pagar(&self) -> String {
        return format!("{} esta pagando", self.name);
    }
    
}

impl Trabalhar for Pessoa {
    fn trabalhar(&self, horas: u32) -> String {
        return format!("{} esta trabalhando por {} horas", self.name, horas);
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
    print!("{}",pessoa2.trabalhar(8));
    print!("{}",pessoa2.jogar("Futebol"));
}
