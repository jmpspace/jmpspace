import Html exposing (..)
import Html.App as Html
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Binary.ArrayBuffer
import WebSocket
import WebSocket.LowLevel

import SpaceServer.Session
import SpaceServer.Server

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

type alias UnauthenticatedState =
  { username: String
  , loginFailed: Bool
  }

type alias ActiveState =
  { username: String
  }

-- MODEL

type Model
  = Unauthenticated UnauthenticatedState
  | LoggedIn ActiveState

init : (Model, Cmd Msg)
init =
  (Unauthenticated <| UnauthenticatedState "" False, Cmd.none)

-- UPDATE

type Msg
  = NoOp
  | UsernameInput String
  | LoginSubmit
  | LoginResponse
  | ServerMessage SpaceServer.Server.Response

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case (model, msg) of
    (Unauthenticated state, UsernameInput newUsername) ->
      (Unauthenticated { state | username = newUsername }, Cmd.none)
    (Unauthenticated {username}, LoginSubmit) ->
      let
        loginRequest = SpaceServer.Session.LoginRequest username
        msg =
          loginRequest |>
          SpaceServer.Server.Request_oneof_request_login >>
          SpaceServer.Server.Request >>
          SpaceServer.Server.marshalRequest >>
          SpaceServer.Server.encodeRequest
      in (model, WebSocket.send echoServer <| WebSocket.LowLevel.ArrayBuffer msg)
    (Unauthenticated state, ServerMessage {response}) ->
      case response of
        SpaceServer.Server.Response_oneof_response_login {response} ->
          case response of
            SpaceServer.Session.LoginResponse_oneof_response_failure failure ->
              (Unauthenticated { state | loginFailed = True }, Cmd.none)
            SpaceServer.Session.LoginResponse_oneof_response_success success ->
              (LoggedIn { username = success.playerName}, Cmd.none)
        _ -> (model, Cmd.none)
    _ -> (model, Cmd.none)

-- SUBSCRIPTIONS

subscriptions : Model -> Sub Msg
subscriptions model =
  WebSocket.listen echoServer (\messageData -> case messageData of
    WebSocket.LowLevel.ArrayBuffer buf -> buf |> SpaceServer.Server.decodeResponse >> SpaceServer.Server.unmarshalResponse >> ServerMessage
    _ -> NoOp)

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
activeView {username} = text <| "Logged in as " ++ username