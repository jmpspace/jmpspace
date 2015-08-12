
module Converter where

import Json.Encode exposing (Value)

import Contracts.Common exposing (Bytes)
import Contracts.Controls 
import Types exposing (..)

import Native.Converter

makeContractControls : Controls -> Contracts.Controls.Controls
makeContractControls c = case c of
  Brakes -> Contracts.Controls.Controls_Brakes
  Active groups -> Contracts.Controls.Controls_Active { groups = groups }

marshalControls : Contracts.Controls.Controls -> Value
marshalControls = Native.Converter.marshalControls
