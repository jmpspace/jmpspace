
module Contracts.World where

import Contracts.Ship exposing (Ship)

type alias Snapshot = {
  ships: List Ship
}

type GameUpdate = GameUpdate_snapshot Snapshot | GameUpdate_focusEntityId Int
