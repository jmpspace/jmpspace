import Html exposing (..)
import Html.App as Html
import Html.Attributes exposing (..)
import Html.Events exposing (..)

-- Need to get around native issues

import Native.Binary.ArrayBuffer
import Binary.ArrayBuffer

import WebSocket
import WebSocket.LowLevel

--import Native.Shim

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



-- MODEL


type alias Model =
  { input : String
  , messages : List WebSocket.MessageData
  }


init : (Model, Cmd Msg)
init =
  (Model "" [], Cmd.none)

-- UPDATE

type Msg
  = Input String
  | Send
  | NewMessage WebSocket.MessageData

update : Msg -> Model -> (Model, Cmd Msg)
update msg {input, messages} =
  case msg of
    Input newInput ->
      (Model newInput messages, Cmd.none)

    Send ->
      (Model "" messages, WebSocket.send echoServer input)

    NewMessage messageData ->
      (Model input (messageData :: messages), Cmd.none)

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
    , div [] (List.map viewMessage (List.reverse model.messages))
    ]

viewMessage : WebSocket.MessageData -> Html msg
viewMessage msg = case msg of
  (WebSocket.LowLevel.String str) ->
    div [] [ text str ]
  (WebSocket.LowLevel.ArrayBuffer _) ->
    div [] [ text "ArrayBuffer" ]
