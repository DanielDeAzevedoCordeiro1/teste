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

impl Dancar for Pessoa {
    fn dancar(&self) -> String {
        return format!("{} esta dancando", self.name);
    }
}

impl Saltar for Pessoa {
    fn saltar(&self) -> String {
        return format!("{} esta saltando", self.name);
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
}
