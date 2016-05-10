package main

import (
	"golang.org/x/net/websocket"
	"math/rand"
)

type gameAction struct {
	u, v int
}

type Action struct {
	client int32
	buf    []byte
}

type Snapshot struct {
	buf []byte
}

type registration struct {
	client   int32
	register bool
	receiver chan Snapshot
}

func actionServer(connects chan Connect, actions chan Action, snapshots chan Snapshot) func(*websocket.Conn) {

	registrations := make(chan registration)
	listeners := make(map[int32]registration)

	go func() {

		for {

			select {

			case registration := <-registrations:

				if registration.register {
					listeners[registration.client] = registration
				} else {
					delete(listeners, registration.client)
				}

			case snapshot := <-snapshots:

				for _, registration := range listeners {
					registration.receiver <- snapshot
				}

			}

		}

	}()

	actionHandler := func(ws *websocket.Conn) {

		client := rand.Int31()
		finished := make(chan int)

		connect := Connect{
			client:   client,
			finished: finished}

		connects <- connect
		<-finished // TODO capture entity value

		rcvSnapshot := make(chan Snapshot)

		registrations <- registration{
			client:   client,
			register: true,
			receiver: rcvSnapshot}

		defer func() {
			registrations <- registration{
				client:   client,
				register: false,
				receiver: rcvSnapshot}
		}()

		go func() {

			for {

				snapshot := <-rcvSnapshot
				err := websocket.Message.Send(ws, snapshot.buf)
				check(err)

			}

		}()

		for {

			var buf []byte
			err := websocket.Message.Receive(ws, &buf)
			check(err)

			action := Action{
				client: client,
				buf:    buf}

			actions <- action

		}

	}

	return actionHandler

}
