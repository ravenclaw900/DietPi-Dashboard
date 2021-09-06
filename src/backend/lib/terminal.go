package lib

import (
	"encoding/json"
	"log"
	"net/http"
	"os/exec"
	"sync"

	"github.com/creack/pty"
	"github.com/gorilla/websocket"
)

type TTYSize struct {
	Cols uint16 `json:"cols"`
	Rows uint16 `json:"rows"`
}

func ServeTerminal(w http.ResponseWriter, r *http.Request) {
	log.Println("Request to /ws/term (terminal)")
	c, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Println("Couldn't upgrade connection to websockets:", err)
		return
	}
	cmd := exec.Command("bash")
	tty, err := pty.Start(cmd)
	if err != nil {
		log.Println("Couldn't open pseudoterminal:", err)
		return
	}
	var waiter sync.WaitGroup
	waiter.Add(1)
	go func() {
		for {
			_, msg, err := c.ReadMessage()
			if err != nil {
				log.Println("Couldn't get data from frontend:", err)
				waiter.Done()
				break
			}
			if string(msg[:4]) == "size" {
				ttySize := &TTYSize{}
				err := json.Unmarshal(msg[4:], ttySize)
				if err != nil {
					log.Println("Couldn't unmarshal JSON:", err)
					waiter.Done()
				}
				pty.Setsize(tty, &pty.Winsize{
					Rows: ttySize.Rows,
					Cols: ttySize.Cols,
				})
				continue
			}
			tty.Write(msg)
		}
	}()
	go func() {
		for {
			buffer := make([]byte, 256)
			readLength, err := tty.Read(buffer)
			if err != nil {
				log.Println("Couldn't get data from TTY:", err)
				break
			}
			c.WriteMessage(websocket.BinaryMessage, buffer[:readLength])
		}
	}()
	waiter.Wait()
	_, err = cmd.Process.Wait()
	if err != nil {
		log.Println("Couldn't gracefully stop shell, killing:", err)
		err = cmd.Process.Kill()
		if err != nil {
			log.Println("Couldn't kill shell:", err)
		}
	}
	err = tty.Close()
	if err != nil {
		log.Println("Couldn't stop pseudoterminal:", err)
	}
	c.Close()
}
