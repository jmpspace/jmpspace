
module Contracts.Ship where

type alias Vessel = {
  width: Float,
  lenght: Float
}

type alias FuelTank = {
  radius: Float,
  length: Float
}

type alias Engine = {
  radius: Float,
  length: Float,
  group: Int
}

type Part = Part_Vessel Vessel | Part_FuelTank FuelTank | Part_Engine Engine

type alias Beam = {
  length: Float
}

type Attachment = Attachment_Part Part | Attachment_Beam Beam

type alias Attach = {
  identity: Int,
  location: Float,
  rotation: Float,
  attachment: Attachment
}
  
type alias Structure = {
  attachments: List Attachment
}
