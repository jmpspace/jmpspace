package main

import (
	"github.com/hoisie/web"
	"net/http"
)

func check(err error) {
	if err != nil {
		panic(err.Error())
	}
}

func main() {

	server := web.NewServer()

	server.Get("/(.*)", http.FileServer(http.Dir("../client/site")))

	server.Get("/chat", chatServer())
	server.Get("/action", actionServer())

	server.Run(":8080")

}
