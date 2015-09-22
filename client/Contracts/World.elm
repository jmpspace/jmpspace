
module Contracts.World where

import Contracts.Ship exposing (Structure)

type alias Snapshot = {
  ships: List Structure
}
