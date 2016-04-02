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

type Connect struct {
	client   int32
	finished chan int
}

func main() {

	connects := make(chan Connect)
	actions := make(chan Action)
	snapshots := make(chan Snapshot)

	http.Handle("/", http.FileServer(http.Dir("site")))
	http.Handle("/action", websocket.Handler(actionServer(connects, actions, snapshots)))

	log.Print("Starting server on port 8080")

	log.Fatal(http.ListenAndServe(":8080", nil))

}
