use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread; // Biblioteca para lidar com concorrência (threads)
use crate::logger; // Importa o módulo de logging para registrar os ataques

// Inicia o servidor TCP e escuta por conexões na porta especificada
pub fn start_listener(address: &str) {
    // Tenta fazer o bind na porta. O "expect" encerra o programa com uma mensagem de erro clara se falhar.
    let listener = TcpListener::bind(address).expect("Falha ao iniciar o listener");
    println!("[*] Sensor escutando em {}", address);
    
    // Loop infinito aguardando novas conexões. 'incoming()' bloqueia a execução até que um novo cliente tente se conectar
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                /*Para cada nova conexão bem-sucedida, cria-se uma nova thread.
                Isso impede que um atacante trave o sensor mantendo a conexão aberta.
                A palavra-chave 'move' transfere a posse (ownership) da variável 'stream' da thread
                principal para dentro da thread recém-criada.*/
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                /*Erros aqui geralmente são problemas de rede efêmeros (ex: handshake falhou).
                 Apenas estará logando o erro; não derruba o sensor inteiro por uma falha isolada.*/
                eprintln!("[-] Erro: {}", e);
            }
        }
    }
}

// Processa a conexão individual do atacante isoladamente em sua própria thread.
fn handle_connection(mut stream: TcpStream) {
    // Tenta extrair o endereço de rede do atacante (peer)
    if let Ok(peer_addr) = stream.peer_addr() {
        let ip = peer_addr.ip();
        println!("[*] IP: {}", ip);
        
        /* Cria um buffer alocado na stack de 1024 bytes (1KB).
        Isso limita o quanto será lido, protegendo o sensor contra ataques de esgotamento de memória.*/
        let mut buffer = [0; 1024];
        let mut payload_final = String::new();

        /*Tenta ler os dados que o atacante enviou logo após conectar.
        'read' preenche o buffer e retorna a quantidade de bytes efetivamente lidos.*/
        if let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read > 0 {
                /*Converte os bytes recebidos em texto legível.
                Usa o 'from_utf8_lossy' porque payloads maliciosos frequentemente contém bytes que não são UTF-8
                válidos (ex: shellcodes). Se usasse uma conversão estrita, o programa entraria em pânico (crash). O lossy
                substitui os erros por ''.*/
                let payload = String::from_utf8_lossy(&buffer[..bytes_read]);
                
                // Exibe o payload capturado limpando quebra de linha (trim).
                println!("Payload: {}", payload.trim());
                
                // Salva na variável externa para o logger conseguir acessar depois
                payload_final = payload.trim().to_string();
            }
        }
        
        // Chama a função para salvar no arquivo JSONL
        logger::log_attack(ip.to_string(), payload_final);
    }
}