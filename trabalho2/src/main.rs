use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use rand::Rng;

struct Mesa {
    estado_hashis: Mutex<Vec<bool>>, // true = hashi disponível
    condicao: Condvar,
}

impl Mesa {
    fn new(n: usize) -> Mesa {
        Mesa {
            estado_hashis: Mutex::new(vec![true; n]),
            condicao: Condvar::new(),
        }
    }

    fn pegar_hashis(&self, id: usize, nome: &str) {
        let (left, right) = (id, (id + 1) % 5); // Hashi esquerdo e direito
        let mut hashis = self.estado_hashis.lock().unwrap();
        let mut rng = rand::thread_rng();
    
        loop {
            // Verifica se o hashi da esquerda está livre
            if hashis[left] {
                hashis[left] = false; // Pega o hashi da esquerda
                println!("{} pegou o hashi à esquerda ({}).", nome, left);
    
                // Verifica se o hashi da direita está livre
                if hashis[right] {
                    hashis[right] = false; // Pega o hashi da direita
                    println!("{} pegou o hashi à direita ({}).", nome, right);
                    break; // Ambos os hashis foram adquiridos, saímos do loop
                } else {
                    // Hashi da direita ocupado, libera o hashi da esquerda e espera
                    hashis[left] = true;
                    println!("{} soltou o hashi à esquerda ({}), esperando...", nome, left);
                    thread::sleep(Duration::from_millis(rng.gen_range(100..1000))); // Espera um pouco antes de tentar de novo, evita livelock
                }
            }
    
            // Espera a condicao
            hashis = self.condicao.wait(hashis).unwrap();
        }
    }

    fn soltar_hashis(&self, id: usize, nome: &str) {
        let (left, right) = (id, (id + 1) % 5); // Hashi esquerdo e direito

        let mut hashis = self.estado_hashis.lock().unwrap();

        // Liberando hashi
        hashis[left] = true;
        hashis[right] = true;
        println!("{} soltou os hashis {} (esquerda) e {} (direita).", nome, left, right);

        // notifica os filosfos esperando
        self.condicao.notify_all();
    }
}

fn filosofo(id: usize, nome: &str, mesa: Arc<Mesa>) {
    let mut rng = rand::thread_rng();
    loop {
        // estando pensando
        println!("{} está pensando.", nome);
        thread::sleep(Duration::from_millis(rng.gen_range(3000..8000)));

        // estado com fome
        println!("{} está com fome.", nome);
        mesa.pegar_hashis(id, nome); // Tenta pegar os hashis

        // estado comendo
        println!("{} está comendo.", nome);
        thread::sleep(Duration::from_millis(rng.gen_range(3000..8000)));

        println!("{} terminou de comer e colocou os hashis na mesa.", nome);
        mesa.soltar_hashis(id, nome); // Libera os hashis após comer
    }
}

fn main() {
    let mesa = Arc::new(Mesa::new(5));
    let mut handles = vec![];

    // Nomes dos filósofos
    let nomes = vec!["Antônio Abujamra", "Clóvis", "Cortella", "Leandro Karnal", "Márcia Tiburi"];

    // Cria uma thread para cada filósofo
    for (i, nome) in nomes.iter().enumerate() {
        let mesa = Arc::clone(&mesa);
        let nome = nome.to_string();
        let handle = thread::spawn(move || {
            filosofo(i, &nome, mesa);
        });
        handles.push(handle);
    }

    // Aguarda todas as threads terminarem
    for handle in handles {
        handle.join().unwrap();
    }
}