package lib

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{}

type system struct {
	CPU float64 `json:"cpu"`
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
		_, jsonreq, err := c.ReadMessage()
		if err != nil {
			log.Println("Couldn't get data from frontend:", err)
			break
		} else {
			log.Println("Got data from frontend")
		}
		var req request
		err = json.Unmarshal(jsonreq, &req)
		if err != nil {
			log.Println("Couldn't parse JSON from frontend:", err)
		}
		switch req.Page {
		case "/":
			stats := system{CPU()}
			statsjson, err := json.Marshal(stats)
			if err != nil {
				log.Println("Couldn't marshal JSON from system stats:", err)
			}
			err = c.WriteMessage(websocket.TextMessage, statsjson)
			if err != nil {
				log.Println("Couldn't send message to frontend:", err)
			} else {
				log.Println("Sent data to frontend")
			}
		}
	}
}
