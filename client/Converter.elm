
module Converter where

import Json.Encode exposing (Value)

import Contracts.Common exposing (Bytes)
import Contracts.Controls 
import Contracts.Ship
import Contracts.World
import Native.Converter
import Types exposing (..)

-- TODO no good reason for this, deprecate the original Controls form Types

makeContractControls : Controls -> Contracts.Controls.Controls
makeContractControls c = case c of
  Brakes -> Contracts.Controls.Controls_brakes {}
  Active groups -> Contracts.Controls.Controls_active { groups = groups }

-- TODO not actually value
marshalControls : Contracts.Controls.Controls -> Value
marshalControls = Native.Converter.marshalControls

-- TODO not actually value
unmarshalSnapshot : Value -> Contracts.World.Snapshot
unmarshalSnapshot = Native.Converter.unmarshalSnapshot
