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

impl Dancar for Pessoa {
    fn dancar(&self) -> String {
        return format!("{} esta dancando", self.name);
    }
}


fn main(){


    let pessoa = Pessoa::new("Daniel".to_string(), 23);
    pessoa.to_string();
    print!("{}",pessoa.dancar());
}
