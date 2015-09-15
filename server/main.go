package main

import (
	"golang.org/x/net/websocket"
	"log"
	"net/http"
	"time"
)

func check(err error) {
	if err != nil {
		panic(err.Error())
	}
}

type Connect struct {
	client   int
	finished chan int
}

func main() {

	hello_sim()
	sim := build_world()

	connects := make(chan Connect)
	ticks := make(chan interface{})
	actions := make(chan Action)
	snapshots := make(chan Snapshot)

	go func() {

		for {

			select {

			case connect := <-connects:
				entity := connect_client(sim, connect.client)
				connect.finished <- entity

			case <-ticks:
				update_world(sim)
				buf := snapshot_world(sim)
				snapshots <- Snapshot{buf: buf}

			case action := <-actions:
				apply_action(sim, action.client, action.buf)

			}

		}

	}()

	go func() {
		for {
			time.Sleep(50 * time.Millisecond)
			ticks <- nil
		}
	}()

	http.Handle("/", http.FileServer(http.Dir("client/site")))
	http.Handle("/action", websocket.Handler(actionServer(connects, actions, snapshots)))

	log.Print("Starting server on port 8080")

	log.Fatal(http.ListenAndServe(":8080", nil))

}
