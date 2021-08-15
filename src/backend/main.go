package main

import (
	"embed"
	"fmt"
	"log"
	"net/http"
)

//go:embed public
var dir embed.FS

func main() {
	http.HandleFunc("/", serveFile)

	log.Println("Starting server on port 8080...")

	log.Fatal(http.ListenAndServe(":8080", nil))
}

func serveFile(w http.ResponseWriter, r *http.Request) {
	log.Printf("Request to %s\n", r.URL.Path)
	data, err := dir.ReadFile("public/index.html")
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		log.Printf("Error, couldn't load HTML file: %s\n", err)
	}
	fmt.Fprint(w, string(data))
}
