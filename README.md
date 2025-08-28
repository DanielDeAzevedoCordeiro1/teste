# Rust Book - Projeto de Aprendizado

Este é um projeto simples em Rust para demonstrar conceitos básicos da linguagem, incluindo structs, implementações, traits e formatação de strings.

## Descrição

O projeto implementa uma estrutura `Pessoa` com funcionalidades básicas e um trait `Dancar` que demonstra como adicionar comportamentos específicos a tipos personalizados.

### Funcionalidades

- **Struct Pessoa**: Representa uma pessoa com nome e idade
- **Trait Dancar**: Define um comportamento de dança que pode ser implementado por diferentes tipos
- **Métodos personalizados**: Criação de instâncias e exibição de informações

## Estrutura do Código

### Struct Pessoa
```rust
struct Pessoa {
    name: String,
    age: u8
}
```

A struct `Pessoa` contém:
- `name`: Nome da pessoa (String)
- `age`: Idade da pessoa (u8)

### Implementações

#### Métodos da Pessoa
- `new(name: String, age: u8) -> Pessoa`: Construtor para criar uma nova instância
- `to_string(&self)`: Exibe as informações da pessoa no console

#### Trait Dancar
```rust
trait Dancar {
    fn dancar(&self) -> String;
}
```

O trait `Dancar` é implementado para `Pessoa` e retorna uma string indicando que a pessoa está dançando.

## Como Executar

### Pré-requisitos
- Rust instalado (versão 1.0 ou superior)
- Cargo (gerenciador de pacotes do Rust)

### Executando o projeto

1. Clone ou baixe o projeto
2. Navegue até o diretório do projeto:
   ```bash
   cd rust-book
   ```

3. Execute o projeto:
   ```bash
   cargo run
   ```

### Compilando o projeto

Para apenas compilar sem executar:
```bash
cargo build
```

Para compilar em modo release (otimizado):
```bash
cargo build --release
```

## Exemplo de Saída

Quando executado, o programa produz a seguinte saída:
```
Nome: Daniel   Idade: 23
Daniel esta dancando
```

## Estrutura do Projeto

```
rust-book/
├── Cargo.toml          # Configurações do projeto
├── Cargo.lock          # Lock file das dependências
├── src/
│   └── main.rs         # Código fonte principal
├── target/             # Diretório de build (gerado automaticamente)
└── README.md           # Este arquivo
```

## Conceitos Demonstrados

- **Structs**: Definição de tipos de dados personalizados
- **Implementação de métodos**: Adição de funcionalidades a structs
- **Traits**: Definição de comportamentos compartilhados
- **Ownership**: Uso de `String` e referências (`&self`)
- **Formatação de strings**: Uso de `println!`, `print!` e `format!`


## Licença

Este projeto é para fins educacionais e de aprendizado da linguagem Rust.
