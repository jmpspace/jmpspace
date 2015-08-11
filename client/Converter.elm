
module Converter where

import Json.Encode exposing (..)

import Types exposing (..)

ctorField : String
ctorField = "_ctor"

activeField : String
activeField = "_active"

encodeControls : Controls -> Value
encodeControls c = 
  case c of
    Brakes -> object [ (ctorField, string "Brakes") ]
    Active ecs -> object [
      (ctorField, string "Active"),
      (activeField, list <| List.map int ecs)
      ]

