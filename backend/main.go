package main

import (
	"fmt"
	"log"
	"net/http"

	//Importa o pacote de handlers que foi criado.
	"backend/handlers"
)

func main() {
	// Define a rota e vincula à função criada
	http.HandleFunc("/api/alerts", handlers.HandleAlerts)

	port := ":8080"
	fmt.Printf("[*] Backend Central escutando na porta%s...\n", port)

	/* Inicia o servidor HTTP e trava a execução aqui. Se der erro (ex: porta ocupada),
	o log.Fatal encerra o programa*/
	log.Fatal(http.ListenAndServe(port, nil))
}
