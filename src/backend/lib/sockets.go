package lib

import (
	"log"
	"net/http"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{}

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

func readSocket(c *websocket.Conn, m chan<- request, n chan<- int) {
	var req request
	firstmessage := true
	for {
		err := c.ReadJSON(&req)
		if err != nil {
			log.Println("Couldn't get data from frontend:", err)
			close(m)
			close(n)
			break
		}
		if !firstmessage {
			n <- 0
		} else {
			firstmessage = false
		}
		m <- req
	}
}

func ServeWebsockets(w http.ResponseWriter, r *http.Request) {
	log.Println("Request to /ws (websockets)")
	c, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Println("Couldn't upgrade connection to websockets:", err)
		return
	}
	m := make(chan request)
	n := make(chan int)
	go readSocket(c, m, n)
	defer c.Close()
	for i := range m {
		switch i.Page {
		case "/":
		main:
			for {
				stats := system{CPU(), RAM(), Swap()}
				err := c.WriteJSON(stats)
				if err != nil {
					log.Println("Couldn't send message to frontend:", err)
				}
				select {
				case <-n:
					break main
				default:
				}
			}
		case "/process":
		process:
			for {
				processes := processlist{Processes()}
				err := c.WriteJSON(processes)
				if err != nil {
					log.Println("Couldn't send message to frontend:", err)
				}
				select {
				case <-n:
					break process
				default:
				}
			}
		}
	}
}
