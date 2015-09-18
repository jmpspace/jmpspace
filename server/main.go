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
				log.Print("Starting connect")
				entity := connect_client(sim, connect.client)
				connect.finished <- entity
				log.Print("Finished connect")

			case <-ticks:
				log.Print("Starting tick")
				update_world(sim)
				buf := snapshot_world(sim)
				snapshots <- Snapshot{buf: buf}
				log.Print("Finished tick")

			case action := <-actions:
				log.Print("Starting action")
				apply_action(sim, action.client, action.buf)
				log.Print("Finished action")
			}

		}

	}()

	/*
		go func() {
			for {
				time.Sleep(50 * time.Millisecond)
				ticks <- nil
			}
		}()
	*/

	http.Handle("/", http.FileServer(http.Dir("site")))
	http.Handle("/action", websocket.Handler(actionServer(connects, actions, snapshots)))

	log.Print("Starting server on port 8080")

	log.Fatal(http.ListenAndServe(":8080", nil))

}
