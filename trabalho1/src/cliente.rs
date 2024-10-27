use std::io::{self, Write, Read};
use std::net::{Ipv4Addr, SocketAddrV4, Shutdown, TcpStream};

pub fn main() {
    println!("Hello Client!");

    // Solicita ao usuário o IP e a porta do provedor
    println!("Digite o IP do servidor (exemplo: 127.0.0.1): ");
    let mut ip_input = String::new();
    io::stdin().read_line(&mut ip_input).expect("Erro ao ler o IP");
    let ip = ip_input.trim().parse::<Ipv4Addr>().expect("IP inválido"); // Faz o parse do IP

    println!("Digite a porta do servidor: ");
    let mut port_input = String::new();
    io::stdin().read_line(&mut port_input).expect("Erro ao ler a porta");
    let port = port_input.trim().parse::<u16>().expect("Porta inválida"); // Faz o parse da porta

    let server_addr = SocketAddrV4::new(ip, port);

    match TcpStream::connect(server_addr) {
        Ok(mut stream) => {
            println!("Conectado ao servidor: {:?}", stream.peer_addr().unwrap());

            println!("Digite a mensagem para enviar ao servidor (ou '#END#' para encerrar):");

            let mut message = String::new();
            io::stdin().read_line(&mut message).expect("Erro ao ler a mensagem");
            let message = message.trim(); 

            match stream.write_all(message.as_bytes()) {
                Ok(_) => {
                    println!("Mensagem enviada com sucesso!");

                    let mut buffer = [0; 128];
                    match stream.read(&mut buffer) {
                        Ok(size) => {
                            if size > 0 {
                                println!("Resposta recebida: {}", String::from_utf8_lossy(&buffer[..size]));
                            }
                        }
                        Err(e) => println!("Erro ao ler do servidor: {}", e),
                    }

                    if message == "#END#" {
                        stream.shutdown(Shutdown::Both).expect("Erro ao encerrar conexão");
                    }
                }
                Err(e) => eprintln!("Erro ao enviar a mensagem: {}", e),
            }
        }
        Err(e) => {
            println!("Não foi possível conectar ao servidor: {}", e);
        }
    }
}