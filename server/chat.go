package main

import (
	"fmt"
	"golang.org/x/net/websocket"
	"log"
)

func echo(ws *websocket.Conn) {

	fmt.Println("Opened    websocket")

	for {

		var msg string
		var err error

		err = websocket.Message.Receive(ws, &msg)
		check(err)

		fmt.Println(msg)

		err = websocket.Message.Send(ws, msg)
		check(err)

	}

	fmt.Println("Finished    copying    websocket")
}

func chatServer() func(*websocket.Conn) {

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

				for l := range listeners {

					l <- msg

				}

			}

		}

	}()

	chatHandler := func(ws *websocket.Conn) {

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
			log.Print("message=\"" + msg + "\"")
			check(err)

			broadcast <- msg

		}

	}

	return chatHandler

}
