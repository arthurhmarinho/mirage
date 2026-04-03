mod network;
mod logger;
mod config;

fn main() {
    // 1. Carrega as configurações (variáveis de ambiente ou padrão)
    let app_config = config::Config::load();

    // 2. Monta o endereço final (ex: "0.0.0.0:2222")
    let bind_address = app_config.get_bind_address();

    // 3. Inicia o sensor passando o endereço configurado
    network::start_listener(&bind_address);
}