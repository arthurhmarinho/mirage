use std::env;

pub struct Config {
    pub host: String,
    pub port: String,
}

impl Config {
    // Carrega as configurações das variáveis de ambiente ou usa os padrões
    pub fn load() -> Self {
        // Tenta ler SENSOR_HOST, se falhar, usa "0.0.0.0"
        let host = env::var("SENSOR_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        
        // Tenta ler SENSOR_PORT, se falhar, usa "2222"
        let port = env::var("SENSOR_PORT").unwrap_or_else(|_| "2222".to_string());

        Config { host, port }
    }
    
    // Retorna a string formatada para o listener TCP (ex: "0.0.0.0:2222")
    pub fn get_bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}