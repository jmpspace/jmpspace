import Html exposing (..)
import Html.App as Html
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Binary.ArrayBuffer
import WebSocket
import WebSocket.LowLevel

import SpaceServer.Session


main =
  Html.program
    { init = init
    , view = view
    , update = update
    , subscriptions = subscriptions
    }


echoServer : String
echoServer =
  "ws://localhost:8001"

type alias ActiveState =
  { userName: String
  }

-- MODEL

type Model
  = Unauthenticated String
  | LoggedIn ActiveState

init : (Model, Cmd Msg)
init =
  (Unauthenticated "", Cmd.none)

-- UPDATE

type Msg
  = UsernameInput String
  | LoginSubmit
  | LoginResponse
  | NewMessage WebSocket.MessageData

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case model of
    Unauthenticated username ->
      case msg of
        UsernameInput newUsername -> (Unauthenticated newUsername, Cmd.none)
        LoginSubmit ->
          let
            request = SpaceServer.Session.LoginRequest username
            msg = SpaceServer.Session.encodeLoginRequest << SpaceServer.Session.marshalLoginRequest <| request
          in (model, WebSocket.send echoServer <| WebSocket.LowLevel.ArrayBuffer msg)
        _ -> (model, Cmd.none)
    LoggedIn loginResponse ->
      (model, Cmd.none)

-- SUBSCRIPTIONS

subscriptions : Model -> Sub Msg
subscriptions model =
  WebSocket.listen echoServer NewMessage

-- VIEW

view : Model -> Html Msg
view model =
  case model of
    Unauthenticated username -> loginView
    LoggedIn activeState -> activeView activeState

loginView : Html Msg
loginView =
  div []
    [ input [onInput UsernameInput] []
    , button [onClick LoginSubmit] [text "Login"]
    ]

activeView : ActiveState -> Html Msg
activeView {userName} =
  div []
    [ ]