package main

import (
	"embed"
	"fmt"
	"io/fs"
	"log"
	"net/http"

	"github.com/gorilla/websocket"
	"github.com/ravenclaw900/DietPi-Dashboard/lib"
)

//go:embed public
var dir embed.FS

var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
}

func main() {
	http.HandleFunc("/", serveHTML)

	http.HandleFunc("/favicon.png", serveFavicon)

	dirFS, err := fs.Sub(dir, "public")

	if err != nil {
		log.Fatal("Couldn't open public folder")
	}

	http.Handle("/build/", http.FileServer(http.FS(dirFS)))

	log.Println("Starting server on port 8080...")

	log.Fatal(http.ListenAndServe(":8080", nil))
}

func serveHTML(w http.ResponseWriter, r *http.Request) {
	log.Printf("Request to %s\n", r.URL.Path)

	data, err := dir.ReadFile("public/index.html")
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		log.Printf("Error, couldn't load HTML file: %s\n", err)
	}
	fmt.Fprint(w, string(data))

	log.Println("Setting up websocket connection")

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Printf("Couldn't establish websocket connection: %s\n", err)
		return
	}

	log.Println(lib.CPU())

}

func serveFavicon(w http.ResponseWriter, r *http.Request) {
	log.Printf("Request to favicon, this usually means a page change if on its own")

	data, err := dir.ReadFile("public/favicon.png")
	if err != nil {
		log.Println("Error, couldn't load favicon.png")
	}
	fmt.Fprint(w, string(data))
}
