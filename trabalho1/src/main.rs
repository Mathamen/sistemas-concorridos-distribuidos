use std::io::{self, Write};

// import da main do cliente
mod cliente;

//import da main do provedor
mod provedor;



fn main() {
    println!("Você será um provedor ou cliente? 1 para provedor, 2 para cliente");
    io::stdout().flush().unwrap();


    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");


    // tratando aqui para poder gerar a lógica sem ter que criar 2 projetos. Fica mais fácil
    match input.trim().parse::<i32>() {
        Ok(1) => provedor::main(),
        Ok(2) => cliente::main(),
        _ => println!("Entrada inválida! Por favor, insira 1 para provedor ou 2 para cliente. A aplicação irá encerrar. Adeus."),
    }
}

