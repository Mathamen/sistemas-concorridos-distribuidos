use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream, Shutdown};
use std::thread;

const ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST; 
const PORT: u16 = 8000;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 60]; 
    while match stream.read(&mut data) {
        Ok(size) if size > 0 => {
            let received_message = String::from_utf8_lossy(&data[0..size]).trim().to_string();
            println!("Received message: {}", received_message); // Apenas log.

            let thread_id = thread::current().id();
            
            // Resposta ao cliente
            let response = if received_message == "marco" {
                // Easter egg
                format!("polo (Serviço prestado pela thread {:?})", thread_id)
            } else if received_message == "Pode me prestar um serviço?" {
                format!("Serviço entregue! Serviço prestado pela thread {:?}", thread_id)
            } else {
                format!("Solicitação inválida pela thread {:?}", thread_id)
            };

            // Envia resposta ao cliente
            if let Err(e) = stream.write_all(response.as_bytes()) {
                println!("Falha ao retornar para o cliente: {}", e);
                false
            } else {
                true
            }
        }
        Ok(_) => {
            println!("Cliente desconectado em: {}", stream.peer_addr().unwrap());
            false
        }
        Err(e) => {
            println!("Erro: {}. Terminando a conexão {}", e, stream.peer_addr().unwrap());
            if let Err(e) = stream.shutdown(Shutdown::Both) {
                println!("Erro ao fechar a conexão: {}", e);
            }
            false
        }
    } {}
}

pub fn main() {
    println!("Modo provedor");
    let listener = TcpListener::bind(SocketAddrV4::new(ADDR, PORT)).unwrap();
    let local_addr = listener.local_addr().unwrap(); // Obtem o IP e porta do servidor
    println!("Servidor escutando em: {}", local_addr); // Mostra o IP e porta

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Nova conexão de: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(err) => println!("Falha na conexão: {:?}", err),
        }
    }

    println!("Servidor encerrando.");
}