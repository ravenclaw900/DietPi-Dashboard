package lib

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{}

type system struct {
	CPU  int `json:"cpu"`
	RAM  int `json:"ram"`
	Swap int `json:"swap"`
}

func ServeWebsockets(w http.ResponseWriter, r *http.Request) {
	log.Println("Request to /ws (websockets)")
	stats := system{CPU(), RAM(), Swap()}
	statsjson, err := json.Marshal(stats)
	if err != nil {
		log.Println("Couldn't marshal JSON from system stats:", err)
	}
	c, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Println("Couldn't upgrade connection to websockets:", err)
		return
	}
	defer c.Close()
	err = c.WriteMessage(websocket.TextMessage, statsjson)
	if err != nil {
		log.Println("Couldn't send message to frontend:", err)
	}
}
