import Binary.ArrayBuffer
import Html exposing (..)
import Html.App as Html
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Platform.Sub exposing (batch)
import Time exposing (every, second)
import WebSocket exposing (listen, send)
import WebSocket.LowLevel exposing (MessageData(..))

import SpaceServer.Session exposing
  ( LoginRequest
  , Ping
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
  , error: Maybe String
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
  | LoginForce
  | LoginResponse
  | ServerMessage SpaceServer.Server.Response
  | PingTick

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
        Response_oneof_response_unauthenticated {error} ->
          (Unauthenticated { username = "", loginFailed = False, error = error }, Cmd.none)
        Response_oneof_response_loggedIn { playerId, playerName } ->
          (LoggedIn { username = playerName}, Cmd.none)
        _ -> unexpectedMessage msg model
    (Unauthenticated state, UsernameInput newUsername) ->
      (Unauthenticated { state | username = newUsername }, Cmd.none)
    (Unauthenticated {username}, LoginSubmit) ->
      let
        cmd = (LoginRequest username False) |> Request_oneof_request_login >> sendRequest
      in (model, cmd)
    (Unauthenticated {username}, LoginForce) ->
          let
            cmd = (LoginRequest username True) |> Request_oneof_request_login >> sendRequest
          in (model, cmd)
    (Unauthenticated state, ServerMessage {response}) ->
      case response of
        Response_oneof_response_unauthenticated {error} ->
          (Unauthenticated { state | loginFailed = True, error = error }, Cmd.none)
        Response_oneof_response_loggedIn { playerId, playerName } ->
          (LoggedIn { username = playerName}, Cmd.none)
        _ -> unexpectedMessage msg model
    (LoggedIn state, ServerMessage {response}) ->
      case response of
        Response_oneof_response_unauthenticated {error} ->
          (Unauthenticated { username = state.username, loginFailed = True, error = error }, Cmd.none)
        Response_oneof_response_pong _ ->
          (model, Cmd.none)
        _ -> unexpectedMessage msg model
    (LoggedIn _, PingTick) ->
      (model, Ping |> Request_oneof_request_ping >> sendRequest)

    _ -> unexpectedMessage msg model

-- SUBSCRIPTIONS

subscriptions : Model -> Sub Msg
subscriptions model =
  let
    socketSub = listen echoServer (\messageData -> case messageData of
      ArrayBuffer buf -> buf |> decodeResponse >> unmarshalResponse >> ServerMessage
      _ -> NoOp)
  in case model of
    LoggedIn _ -> batch [socketSub, every (5 * second) (\_ -> PingTick)]
    _ -> socketSub

-- VIEW

view : Model -> Html Msg
view model =
  case model of
    Unknown -> text "Contacting server (initializing session state)"
    Unauthenticated state -> loginView state
    LoggedIn activeState -> activeView activeState

loginView : UnauthenticatedState -> Html Msg
loginView state =
  div []
    [ text <| toString state.error
    , input [onInput UsernameInput] []
    , button [onClick LoginSubmit] [text "Login"]
    , button [onClick LoginForce] [text "Force Login"]
    ]

activeView : ActiveState -> Html Msg
activeView {username} = text <| "Logged in as " ++ username