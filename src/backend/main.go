package main

import (
	"embed"
	"fmt"
	"io/fs"
	"log"
	"net/http"

	"github.com/ravenclaw900/DietPi-Dashboard/lib"
)

//go:embed public
var dir embed.FS

func main() {
	http.HandleFunc("/", serveHTML)

	http.HandleFunc("/favicon.png", serveFavicon)

	http.HandleFunc("/ws", lib.ServeWebsockets)

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
}

func serveFavicon(w http.ResponseWriter, r *http.Request) {
	log.Printf("Request to favicon, this usually means a page change if on its own")

	data, err := dir.ReadFile("public/favicon.png")
	if err != nil {
		log.Println("Error, couldn't load favicon.png")
	}
	fmt.Fprint(w, string(data))
}
