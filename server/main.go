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
	sim := build_world()

	ticks := make(chan interface{})
	actions := make(chan Action)
	snapshots := make(chan Snapshot)

	go func() {

		for {

			select {

			case tick := <-ticks:
				update_world(sim)
				snapshot := snapshot_world(sim)
				snapshots <- snapshot

			case action := <-actions:
				apply_action(sim, action)

			}

		}

	}()

	go func() {
		for {
			time.sleep(40)
			ticks <- true
		}
	}()

	http.Handle("/", http.FileServer(http.Dir("client/site")))
	http.Handle("/action", websocket.Handler(actionServer()))

	log.Print("Starting server on port 8080")

	log.Fatal(http.ListenAndServe(":8080", nil))

}
