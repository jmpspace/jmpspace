package main

import (
	"golang.org/x/net/websocket"
	"log"
	"net/http"
)

func check(err error) {
	if err != nil {
		panic(err.Error())
	}
}

func main() {

	hello_sim()

	http.Handle("/", http.FileServer(http.Dir("client/site")))
	http.Handle("/chat", websocket.Handler(chatServer()))
	http.Handle("/action", websocket.Handler(actionServer()))

	log.Print("Starting server on port 8080")

	log.Fatal(http.ListenAndServe(":8080", nil))

}
