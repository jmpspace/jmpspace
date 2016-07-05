import Html exposing (..)
import Html.App as Html
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Binary.ArrayBuffer
import WebSocket exposing (listen, send)
import WebSocket.LowLevel exposing (MessageData(..))

import SpaceServer.Session exposing
  ( LoginRequest
  , SessionStateRequest
  )
import SpaceServer.Server exposing
  ( Request
  , Request_oneof_request(..)
  , Response_oneof_response(..)
  , decodeResponse
  , encodeRequest
  , marshalRequest
  , unmarshalResponse )

main =
  Html.program
    { init = init
    , view = view
    , update = update
    , subscriptions = subscriptions
    }


echoServer : String
echoServer = "ws://localhost:8001"

type alias UnauthenticatedState =
  { username: String
  , loginFailed: Bool
  }

type alias ActiveState =
  { username: String
  }

-- MODEL

type Model
  = Unknown
  | Unauthenticated UnauthenticatedState
  | LoggedIn ActiveState

init : (Model, Cmd Msg)
init =
  let
    cmd = SessionStateRequest |> Request_oneof_request_sessionState >> sendRequest
  in (Unknown, cmd)

-- UPDATE

type Msg
  = NoOp
  | UsernameInput String
  | LoginSubmit
  | LoginResponse
  | ServerMessage SpaceServer.Server.Response

unexpectedMessage : Msg -> Model -> (Model, Cmd Msg)
unexpectedMessage msg model = Debug.log ("Unexpected message" ++ toString msg) (model, Cmd.none)

sendRequest : Request_oneof_request -> Cmd msg
sendRequest =
  Request >>
  marshalRequest >>
  encodeRequest >>
  ArrayBuffer >>
  send echoServer

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case (model, msg) of
    (Unknown, ServerMessage {response}) ->
      case response of
        Response_oneof_response_unauthenticated _ ->
          (Unauthenticated { username = "", loginFailed = False }, Cmd.none)
        Response_oneof_response_loggedIn { playerId, playerName } ->
          (LoggedIn { username = playerName}, Cmd.none)
        --_ -> unexpectedMessage msg model
    (Unauthenticated state, UsernameInput newUsername) ->
      (Unauthenticated { state | username = newUsername }, Cmd.none)
    (Unauthenticated {username}, LoginSubmit) ->
      let
        cmd = (LoginRequest username) |> Request_oneof_request_login >> sendRequest
      in (model, cmd)
    (Unauthenticated state, ServerMessage {response}) ->
      case response of
        Response_oneof_response_unauthenticated _ ->
          (Unauthenticated { state | loginFailed = True }, Cmd.none)
        Response_oneof_response_loggedIn { playerId, playerName } ->
          (LoggedIn { username = playerName}, Cmd.none)
        --_ -> unexpectedMessage msg model
    _ -> unexpectedMessage msg model

-- SUBSCRIPTIONS

subscriptions : Model -> Sub Msg
subscriptions model =
  listen echoServer (\messageData -> case messageData of
    ArrayBuffer buf -> buf |> decodeResponse >> unmarshalResponse >> ServerMessage
    _ -> NoOp)

-- VIEW

view : Model -> Html Msg
view model =
  case model of
    Unknown -> text "Contacting server (initializing session state)"
    Unauthenticated username -> loginView
    LoggedIn activeState -> activeView activeState

loginView : Html Msg
loginView =
  div []
    [ input [onInput UsernameInput] []
    , button [onClick LoginSubmit] [text "Login"]
    ]

activeView : ActiveState -> Html Msg
activeView {username} = text <| "Logged in as " ++ username