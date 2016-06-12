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

type SessionState
  = Unauthenticated
  | LoggedIn SpaceServer.Session.LoginResponse

-- MODEL

type alias Model =
  { input : String
  , sessionState : SessionState
  , messages : List WebSocket.MessageData
  }


init : (Model, Cmd Msg)
init =
  (Model "" Unauthenticated [], Cmd.none)

-- UPDATE

type Msg
  = Input String
  | Send
  | SendBinary
  | NewMessage WebSocket.MessageData

update : Msg -> Model -> (Model, Cmd Msg)
update msg ({input, sessionState, messages} as model) =
  case msg of
    Input newInput ->
      ({model | input = newInput}, Cmd.none)

    Send ->
      let
        messageData = WebSocket.LowLevel.String input
      in ({model | input = ""}, WebSocket.send echoServer messageData)

    SendBinary ->
      let
        buffer = Binary.ArrayBuffer.new 10
        messageData = WebSocket.LowLevel.ArrayBuffer buffer
      in ({model | input = ""}, WebSocket.send echoServer messageData)

    NewMessage messageData ->
      ({model | messages = messageData :: messages}, Cmd.none)

-- SUBSCRIPTIONS

subscriptions : Model -> Sub Msg
subscriptions model =
  WebSocket.listen echoServer NewMessage

-- VIEW

view : Model -> Html Msg
view model =
  div []
    [ input [onInput Input] []
    , button [onClick Send] [text "Send"]
    , button [onClick SendBinary] [text "SendBinary"]
    , div [] (List.map viewMessage (List.reverse model.messages))
    ]

viewMessage : WebSocket.MessageData -> Html msg
viewMessage msg = case msg of
  (WebSocket.LowLevel.String str) ->
    div [] [ text <| "String: " ++ str ]
  (WebSocket.LowLevel.ArrayBuffer buf) ->
    div [] [ text <| "ArrayBuffer with length in bytes: " ++ toString  (Binary.ArrayBuffer.byteLength buf)]
