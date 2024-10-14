use std::io::{self, Write, Read};
use std::net::{Ipv4Addr, SocketAddrV4, Shutdown, TcpStream};

const ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST;
const PORT: u16 = 8000;

pub fn main() {
    println!("Hello Client!");

    match TcpStream::connect(SocketAddrV4::new(ADDR, PORT)) {
        Ok(mut stream) => {
            println!("Connected to the server on {:?}", stream.peer_addr().unwrap());

            println!("Digite a mensagem para enviar ao servidor (ou '#END#' para encerrar):");

            let mut message = String::new();
            io::stdin().read_line(&mut message).expect("Failed to read input");
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
                                println!("Response from server: {}", String::from_utf8_lossy(&buffer[..size]));
                            }
                        }
                        Err(e) => println!("Failed to read from server: {}", e),
                    }

                    // Encerra a conexão se a mensagem for "#END#"
                    if message == "#END#" {
                        stream.shutdown(Shutdown::Both).expect("Shutdown failed!");
                    }
                }
                Err(e) => eprintln!("Failed to send message: {}", e),
            }
        }
        Err(e) => {
            println!("Couldn't connect to server: {}", e);
        }
    }
}
