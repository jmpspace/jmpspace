
module WsTasks where

import Task exposing (Task)

import Native.WsTasks

-- Opaque types
type Socket = Socket Socket

open : String -> Task String Socket
open = Native.WsTasks.open

relativeUri : String -> String
relativeUri = Native.WsTasks.relativeUri
