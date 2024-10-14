use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream, Shutdown};
use std::thread;

const ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST; 
const PORT: u16 = 8000;


fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 60]; 
    while match stream.read(&mut data) {
        Ok(size) if size > 0 => {
            let received_message = String::from_utf8_lossy(&data[0..size]);
            println!("Received message: {}", received_message);

            // Easter egg
            let response = if received_message.trim() == "marco" {
                "polo"
            } else {
                "Message received"
            };

            // Enviar resposta
            if let Err(e) = stream.write_all(response.as_bytes()) {
                println!("Falha ao enviar dados ao cliente: {}", e);
                false
            } else {
                true
            }
        }
        Ok(_) => {
            println!("Client disconnected: {}", stream.peer_addr().unwrap());
            false
        }
        Err(e) => {
            println!("Error occurred: {}. Terminating connection with {}", e, stream.peer_addr().unwrap());
            if let Err(e) = stream.shutdown(Shutdown::Both) {
                println!("Erro ao fechar conexão: {}", e);
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
                // Thread para cliente
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(err) => println!("Connection failed: {:?}", err),
        }
    }

    println!("Server shutting down.");
}
