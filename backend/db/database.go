package db

import (
	"database/sql"
	"fmt"
	"log"
	"os"

	"github.com/joho/godotenv"
	_ "github.com/lib/pq" // Importa o driver do PostgreSQL
)

var Conn *sql.DB

func Connect() {
	// Carrega as variáveis de ambiente do arquivo .env
	err := godotenv.Load("../.env")
	if err != nil {
		log.Println("[!]Aviso: Arquivo .env não encontrado. Usando variáveis de ambiente do sistema.")
	}

	// Puxa as credenciais do ambiente
	dbUser := os.Getenv("POSTGRES_USER")
	dbPass := os.Getenv("POSTGRES_PASSWORD")
	dbName := os.Getenv("POSTGRES_DB")

	// Monta a string de conexão dinamicamente
	connStr := fmt.Sprintf("host=localhost port=5433 user=%s password=%s dbname=%s sslmode=disable", dbUser, dbPass, dbName)

	Conn, err = sql.Open("postgres", connStr)
	if err != nil {
		log.Fatal("[-] Erro ao abrir conexão com o banco: ", err)
	}

	err = Conn.Ping()
	if err != nil {
		log.Fatal("[-] Banco de dados não está respondendo: ", err)
	}

	fmt.Println("[+] Conectado ao PostgreSQL com sucesso!")

	createTable()
}

func createTable() {
	query := `
	CREATE TABLE IF NOT EXISTS alerts (
		id SERIAL PRIMARY KEY,
		timestamp BIGINT,
		ip VARCHAR(45),
		payload TEXT
	);
	`
	_, err := Conn.Exec(query)
	if err != nil {
		log.Fatal("[-] Erro ao criar tabela de alertas: ", err)
	}
	fmt.Println("[+] Tabela de 'alerts' pronta para uso.")
}
