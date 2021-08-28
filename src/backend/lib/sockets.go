package lib

import (
	"log"
	"net/http"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
	ReadBufferSize: -1,
}

type system struct {
	CPU  float64 `json:"cpu"`
	RAM  MemData `json:"ram"`
	Swap MemData `json:"swap"`
}

type processlist struct {
	Processes []ProcessData `json:"processes"`
}

type request struct {
	Page string `json:"page"`
}

func ServeWebsockets(w http.ResponseWriter, r *http.Request) {
	log.Println("Request to /ws (websockets)")
	c, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Println("Couldn't upgrade connection to websockets:", err)
		return
	}
	defer c.Close()
	for {
		var req request
		err := c.ReadJSON(&req)
		if err != nil {
			log.Println("Couldn't get data from frontend:", err)
			break
		}
		switch req.Page {
		case "/":
			stats := system{CPU(), RAM(), Swap()}
			err := c.WriteJSON(stats)
			if err != nil {
				log.Println("Couldn't send message to frontend:", err)
			}
		case "/process":
			processes := processlist{Processes()}
			err := c.WriteJSON(processes)
			if err != nil {
				log.Println("Couldn't send message to frontend:", err)
			}
		}
	}
}
