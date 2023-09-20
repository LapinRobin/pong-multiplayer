package main

import (
	"encoding/json"
	"net/http"
	"sync"
	"time"

	"github.com/gorilla/websocket"
)

var (
	clients      = make([]*websocket.Conn, 0)
	clientsMu    sync.Mutex
	canvasHeight float64 = 600
	canvasWidth  float64 = 800
	paddleSpeed  float64 = 10
	paddleHeight float64 = 100
	paddleWidth  float64 = 10
	gameState            = map[string]float64{
		"leftPaddleY":  300,
		"rightPaddleY": 300,
		"ballX":        300,
		"ballY":        300,
		"ballSpeedX":   2,
		"ballSpeedY":   2,
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

func gameLoop() {
	for {
		// update ball position
		gameState["ballX"] += gameState["ballSpeedX"]
		gameState["ballY"] += gameState["ballSpeedY"]

		if gameState["ballY"] <= 0 || gameState["ballY"] >= canvasHeight {
			gameState["ballSpeedY"] = -gameState["ballSpeedY"]
		}

		if gameState["ballX"] <= paddleWidth {
			if gameState["ballY"] > gameState["leftPaddleY"] && gameState["ballY"] < gameState["leftPaddleY"]+paddleHeight {
				// Ball hit the left paddle, bounce it
				gameState["ballSpeedX"] = -gameState["ballSpeedX"]
			} else {
				// Ball missed the left paddle, player 2 wins
				gameState["winner"] = 2
				resetGameState()
			}
		} else if gameState["ballX"] >= canvasWidth-paddleWidth {
			if gameState["ballY"] > gameState["rightPaddleY"] && gameState["ballY"] < gameState["rightPaddleY"]+paddleHeight {
				// Ball hit the right paddle, bounce it
				gameState["ballSpeedX"] = -gameState["ballSpeedX"]
			} else {
				// Ball missed the right paddle, player 1 wins
				gameState["winner"] = 1
				resetGameState()
			}
		}

		// Broadcast updated gameState
		gameStateJSON, _ := json.Marshal(gameState)
		clientsMu.Lock()
		for _, client := range clients {
			if err := client.WriteMessage(websocket.TextMessage, gameStateJSON); err != nil {
				// Handle error
			}
		}
		clientsMu.Unlock()

		time.Sleep(time.Millisecond * 20) // Roughly 60 updates per second
	}
}

func resetGameState() {
	gameState["leftPaddleY"] = 300
	gameState["rightPaddleY"] = 300
	gameState["ballX"] = 300
	gameState["ballY"] = 300
	gameState["ballSpeedX"] = 2
	gameState["ballSpeedY"] = 2
	delete(gameState, "winner")
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
				if direction == "up" && gameState["leftPaddleY"] > 0 {
					gameState["leftPaddleY"] -= paddleSpeed
				} else if direction == "down" && gameState["leftPaddleY"] < canvasHeight-paddleHeight {
					gameState["leftPaddleY"] += paddleSpeed
				}
			} else if playerID == 2 {
				if direction == "up" && gameState["rightPaddleY"] > 0 {
					gameState["rightPaddleY"] -= paddleSpeed
				} else if direction == "down" && gameState["rightPaddleY"] < canvasHeight-paddleHeight {
					gameState["rightPaddleY"] += paddleSpeed
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
	go gameLoop()
	http.HandleFunc("/", handler)
	http.ListenAndServe(":8080", nil)
}
