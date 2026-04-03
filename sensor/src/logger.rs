use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

// Esse macro 'derive' faz o Rust criar o código de conversão para JSON automaticamente
#[derive(Serialize)]
struct AttackLog{
    timestamp: u64,
    ip: String,
    payload: String,
}

// Salva o IP e o payload capturados em um arquivo JSONL.
pub fn log_attack(ip: String, payload: String) {
    // Pega o tempo atual em segundos
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let log_entry = AttackLog {
        timestamp,
        ip,
        payload,
    };

    // Converte a struct para uma string JSON.
    if let Ok(mut json_string) = serde_json::to_string(&log_entry) {
        json_string.push('\n'); // Adiciona quebra de linha para manter o formato JSONL

        // Abre o arquivo em modo 'append' (adiciona ao final sem apagar o que já existe)
        let mut file = OpenOptions::new()
            .create(true) // Cria o arquivo se não existir
            .append(true) // Adiciona no final do arquivo em vez de sobrescrever
            .open("sensor_logs.jsonl")
            .expect("Falha ao abrir o arquivo de log");

        file.write_all(json_string.as_bytes())
            .expect("Falha ao escrever no arquivo de log");
    }
}