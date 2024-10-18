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
            println!("Received message: {}", received_message); // So para log mesmo aqui.

            
            let thread_id = thread::current().id();

            // resposta aqui po 
            let response = if received_message == "marco" {
                // easter egg aqui KKKKKKKK
                format!("polo (Serviço prestado pela thread {:?})", thread_id)
            } else if received_message == "Pode me prestar um serviço?" {
                format!("Serviço entregue! Serviço prestado pela thread {:?}", thread_id)  // Mensagem correta com thread ID
            } else {
                format!("Solicitação inválida pela thread {:?}", thread_id)  // Não está formatado certo. Coloquei para ser exatamente o que pedia na tarefa ou então o easter egg
            };

         
            if let Err(e) = stream.write_all(response.as_bytes()) {
                println!("Falha ao retornar para o cliente: {}", e); // Rust exige tratamento de erro né
                false
            } else {
                true
            }
        }
        Ok(_) => {
            println!("Cliente desconectado em: {}", stream.peer_addr().unwrap()); // So para mostrar as diferentes portas
            false
        }
        Err(e) => {
            println!("Ocorreu o erro: {}. Terminando a conexão {}", e, stream.peer_addr().unwrap());
            if let Err(e) = stream.shutdown(Shutdown::Both) {
                println!("Erro ao fechar conexão: {}", e); // Nunca vi esse erro mas preciso pois ok / err
            }
            false
        }
    } {}
}

pub fn main() {
    println!("Modo provedor");
    let listener = TcpListener::bind(SocketAddrV4::new(ADDR, PORT)).unwrap();
    println!("Listening on {:?}", listener);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // Criando a thread que vai server o cliente aqui
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(err) => println!("Connection failed: {:?}", err),
        }
    }

    println!("Server shutting down.");
}
