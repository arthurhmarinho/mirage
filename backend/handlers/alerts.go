package handlers

import (
	"backend/db"
	"encoding/json"
	"fmt"
	"net/http"
)

type Alert struct {
	Timestamp uint64 `json:"timestamp"`
	IP        string `json:"ip"`
	Payload   string `json:"payload"`
}

func HandleAlerts(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Método não permitido", http.StatusMethodNotAllowed)
		return
	}

	var alert Alert
	err := json.NewDecoder(r.Body).Decode(&alert)
	if err != nil {
		http.Error(w, "Erro ao processar o JSON", http.StatusBadRequest)
		return
	}

	// Insere o alerta diretamente no PostgreSQL
	query := `INSERT INTO alerts (timestamp, ip, payload) VALUES ($1, $2, $3)`
	_, err = db.Conn.Exec(query, alert.Timestamp, alert.IP, alert.Payload)
	if err != nil {
		fmt.Println("[-] Erro ao salvar no banco:", err)
		http.Error(w, "Erro interno do servidor ao salvar dado", http.StatusInternalServerError)
		return
	}

	// Mostra no console apenas confirmando que salvou
	fmt.Printf("[!] ALERTA SALVO NO BANCO DE DADOS:\n")
	fmt.Printf("    Origem: %s\n", alert.IP)
	fmt.Println("---------------------------------------------------")

	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(map[string]string{"message": "Alerta persistido no PostgreSQL"})
}
