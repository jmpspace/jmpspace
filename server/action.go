package main

import (
	"code.google.com/p/go.net/websocket"
	"encoding/json"
)

type gameAction struct {
	u, v int
}

func actionServer() interface{} {

	broadcast := make(chan string)
	register := make(chan (chan string))
	unregister := make(chan (chan string))
	listeners := make(map[(chan string)]bool)

	go func() {

		for {

			select {

			case rcv := <-register:

				listeners[rcv] = true

			case rcv := <-unregister:

				delete(listeners, rcv)

			case msg := <-broadcast:

				//    TODO:    switch    on    adding    a    new    listener

				var action gameAction

				err := json.Unmarshal([]byte(msg), action)
				if err != nil {
					continue
				}

			}

		}

	}()

	actionHandler := func(ws *websocket.Conn) {

		rcv := make(chan string)

		register <- rcv

		defer func() { unregister <- rcv }()

		go func() {

			for {

				msg := <-rcv
				err := websocket.Message.Send(ws, msg)
				check(err)

			}

		}()

		for {

			var msg string
			err := websocket.Message.Receive(ws, &msg)
			check(err)

			broadcast <- msg

		}

	}

	return upgradeWebsocketHandler(websocket.Handler(actionHandler))

}
