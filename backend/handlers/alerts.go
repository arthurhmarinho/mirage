package handlers

import (
	"encoding/json"
	"fmt"
	"net/http"
)

// Alert representa a estrutura exata do JSON enviado pelo sensor Rust
type Alert struct {
	Timestamp uint64 `json:"timestamp"`
	IP        string `json:"ip"`
	Payload   string `json:"payload"`
}

func HandleAlerts(w http.ResponseWriter, r *http.Request) {
	// Garante que só aceite requisições POST
	if r.Method != http.MethodPost {
		http.Error(w, "Método não permitido", http.StatusMethodNotAllowed)
		return
	}

	var alert Alert

	// Decodifica o corpo da requisição (JSON) para o struct Go
	err := json.NewDecoder(r.Body).Decode(&alert)
	if err != nil {
		http.Error(w, "Erro ao processar o JSON", http.StatusBadRequest)
		return
	}

	//No MVP, vai apenas imprimir o alerta no console do backend. Futuramente, pode ser armazenado em um banco de dados ou enviado para um sistema de monitoramento.
	fmt.Printf("[!] NOVO ALERTA RECEBIDO:\n")
	fmt.Printf("   Origem: %s\n", alert.IP)
	fmt.Printf("   Payload: %s\n", alert.Payload)
	fmt.Println("--------------------------------------------------")

	// Responde ao sensor que o dado foi recebido com sucesso (HTTP 201 Created)
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(map[string]string{"message": "Alerta registrado"})

}
