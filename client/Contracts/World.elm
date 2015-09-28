
module Contracts.World where

import Contracts.Ship exposing (Ship)

type alias Snapshot = {
  ships: List Ship
}

type GameState = GameState_snapshot Snapshot | GameState_focusEntityId Int
