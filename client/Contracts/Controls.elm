
module Contracts.Controls where

type alias Active = {
  groups : List Int
}

type Controls = Controls_Brakes | Controls_Active Active
