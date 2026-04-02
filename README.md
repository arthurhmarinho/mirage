# Mirage

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/sensor-Rust-orange.svg)
![Go](https://img.shields.io/badge/backend-Go-00ADD8.svg)
![Docker](https://img.shields.io/badge/deploy-Docker-2496ED.svg)

**Mirage** é um Sistema Distribuído de Honeypots focado em capturar, analisar e visualizar ameaças em tempo real. Projetado para ser leve, seguro e altamente escalável, ele cria "miragens" de serviços vulneráveis para atrair atacantes, registrando seus IPs e payloads maliciosos de forma invisível.

---

## Arquitetura do Sistema

O projeto é dividido em microsserviços bem definidos para garantir isolamento e escalabilidade:

*   **Sensor (Rust):** Serviço ultraleve e com segurança de memória garantida nativamente. Fica exposto na rede simulando portas abertas (ex: SSH, HTTP) e captura os dados da invasão no primeiro contato.
*   **Backend (Go):** API centralizadora de alta performance que recebe os dados de múltiplos sensores simultaneamente, processa os payloads e armazena os logs.
*   **Dashboard (TypeScript):** Interface gráfica para visualização das métricas, exibindo mapas de origem dos ataques e estatísticas de tentativas de invasão em tempo real.
*   **Deploy (Docker):** Toda a infraestrutura é empacotada em contêineres para facilitar o isolamento dos honeypots e o deploy ágil em qualquer ambiente.

---

## Como Começar

### Pré-requisitos

Para rodar o Mirage localmente ou em seu servidor, você precisará apenas do **Docker** e do **Docker Compose** instalados na sua máquina.

### Instalação

1. Clone o repositório:
```bash
git clone [https://github.com/SEU_USUARIO/mirage.git](https://github.com/SEU_USUARIO/mirage.git)
cd mirage
```

2. Suba a infraestrutura completa utilizando o Docker Compose:
```bash
cd deploy
docker-compose up -d --build
```

3. O sistema estará rodando!
   * **Sensor:** Escutando na porta configurada (ex: `2222`)
   * **Dashboard:** Acessível em `http://localhost:3000`

---

## Como Contribuir

O **Mirage** é um projeto de código aberto e todas as contribuições são muito bem-vindas. Se você deseja melhorar a segurança dos sensores, otimizar a API ou melhorar a interface, siga os passos abaixo:

1. Faça um *Fork* do projeto
2. Crie uma *Branch* para sua feature (`git checkout -b feature/NovaFuncionalidade`)
3. Faça o *Commit* das suas alterações (`git commit -m 'feat: Adicionando nova funcionalidade ao sensor'`)
4. Faça o *Push* para a branch (`git push origin feature/NovaFuncionalidade`)
5. Abra um *Pull Request*

Por favor, certifique-se de que seu código segue as boas práticas das linguagens utilizadas e inclua testes sempre que possível.

---

## Licença

Este projeto está sob a licença MIT. Veja o arquivo LICENSE para mais detalhes.
