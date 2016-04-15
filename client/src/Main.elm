
module Main where

import Html exposing (Html, text)
import Task exposing (Task, andThen, succeed)

import WsTasks exposing (open, relativeUri)

main : Html
main = text <| WsTasks.relativeUri "/ws"

port serverThread : Task String ()
port serverThread =
  WsTasks.open (WsTasks.relativeUri "/ws") 
  `andThen` \_ -> succeed ()
