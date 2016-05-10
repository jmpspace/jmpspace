
module Contracts.Controls where

type alias Unit = {}

type alias Active = {
  groups : List Int
}

type Controls = Controls_brakes Unit | Controls_active Active

type alias Build = {
  foo : Int
}

type Action = Action_controls Controls | Action_build Build
