use std::io::{self, Write, Read};
use std::net::{Ipv4Addr, SocketAddrV4, Shutdown, TcpStream};

const ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST;
const PORT: u16 = 8000;

pub fn main() {
    println!("Hello Client!");

    match TcpStream::connect(SocketAddrV4::new(ADDR, PORT)) {
        Ok(mut stream) => {
            println!("Conectado com sucesso!! Servidor {:?}", stream.peer_addr().unwrap());

            println!("Digite a mensagem para enviar ao servidor (ou '#END#' para encerrar):");

            let mut message = String::new();
            io::stdin().read_line(&mut message).expect("Erro ao ler a mensagem"); // rust precisa disso
            let message = message.trim(); // Removendo \n

            // enviar msg
            match stream.write_all(message.as_bytes()) {
                Ok(_) => {
                    println!("Message sent successfully!");

                    // Ler resposta
                    let mut buffer = [0; 128];
                    match stream.read(&mut buffer) {
                        Ok(size) => {
                            if size > 0 {
                                println!("Resposta recebida!! Resposta: {}", String::from_utf8_lossy(&buffer[..size]));
                            }
                        }
                        Err(e) => println!("Erro ao ler do servidor: {}", e), // Nunca vi esse erro KKKKKK, mas de novo, ok / err
                    }

                    // Encerra a conexão se a mensagem for "#END#", só para debug basicamente
                    if message == "#END#" {
                        stream.shutdown(Shutdown::Both).expect("Shutdown failed!"); // mais uma vez rust precisa do expect. tratamento de erro
                    }
                }
                Err(e) => eprintln!("Erro ao enviar a mensagem: {}", e), 
            }
        }
        Err(e) => {
            println!("Não foi possível conectar ao provedor: {}", e);  
        }
    }
}
