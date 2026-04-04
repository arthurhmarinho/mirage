use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

// Estrutura idêntica à que o backend em Go espera
#[derive(Serialize)]
struct Alert {
    timestamp: u64,
    ip: String,
    payload: String,
}

// Envia os dados capturados para o Backend via HTTP POST
pub fn send_alert(ip: String, payload: String) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let alert = Alert { timestamp, ip, payload };
    let url = "http://localhost:8080/api/alerts";

    // Dispara a requisição POST com o JSON embutido
    match ureq::post(url).send_json(alert) {
        Ok(_) => println!("   [+] Alerta enviado com sucesso para o Backend Central"),
        Err(e) => println!("  [-] Erro ao enviar para o Backend: {}", e),
    }
}