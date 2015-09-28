
module Contracts.World where

import Contracts.Ship exposing (Ship)

type alias Snapshot = {
  ships: List Ship
}
