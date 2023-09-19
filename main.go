package main

import (
	"encoding/json"
	"net/http"
	"sync"

	"github.com/gorilla/websocket"
)

var (
	clients   = make([]*websocket.Conn, 0)
	clientsMu sync.Mutex
	gameState = map[string]float64{
		"leftPaddleY":  300,
		"rightPaddleY": 300,
	}
	playerCounter = 0
	playerMap     = make(map[*websocket.Conn]int)
)

var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
	CheckOrigin: func(r *http.Request) bool {
		return true // Allow cross-origin requests for testing
	},
}

func handler(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		// Handle error
		return
	}
	defer conn.Close()

	clientsMu.Lock()
	clients = append(clients, conn)
	playerCounter++
	playerMap[conn] = playerCounter
	clientsMu.Unlock()

	defer func() {
		clientsMu.Lock()
		for i, c := range clients {
			if c == conn {
				clients = append(clients[:i], clients[i+1:]...)
				delete(playerMap, conn)
				break
			}
		}
		clientsMu.Unlock()
	}()

	for {
		messageType, p, err := conn.ReadMessage()
		if err != nil {
			// Handle error
			return
		}

		var data map[string]string
		if err := json.Unmarshal(p, &data); err != nil {
			// Handle JSON decoding error
			return
		}

		playerID := playerMap[conn]
		if data["action"] == "move" {
			direction := data["direction"]
			if playerID == 1 {
				if direction == "up" {
					gameState["leftPaddleY"] -= 5 // Move up by 5 units
				} else if direction == "down" {
					gameState["leftPaddleY"] += 5 // Move down by 5 units
				}
			} else if playerID == 2 {
				if direction == "up" {
					gameState["rightPaddleY"] -= 5 // Move up by 5 units
				} else if direction == "down" {
					gameState["rightPaddleY"] += 5 // Move down by 5 units
				}
			}
		}

		gameStateJSON, _ := json.Marshal(gameState)
		clientsMu.Lock()
		for _, client := range clients {
			if err := client.WriteMessage(messageType, gameStateJSON); err != nil {
				// Handle error
			}
		}
		clientsMu.Unlock()
	}
}

func main() {
	http.HandleFunc("/", handler)
	http.ListenAndServe(":8080", nil)
}
