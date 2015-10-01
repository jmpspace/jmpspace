
module Contracts.Ship where

type alias Vessel = {
  width: Float,
  length: Float
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

type Part = Part_vessel Vessel | Part_fuelTank FuelTank | Part_engine Engine

type alias Beam = {
  length: Float
}

type alias Root = {}

type alias Attach = {
  location: Float,
  rotation: Float
}

type StructureNode = StructureNode_beam Beam | StructureNode_part Part

type StructureLink = StructureLink_root Root | StructureLink_attach Attach

type alias StructureTree = {
  node: StructureNode,
  link: StructureLink
}

type alias EndMarker = {}

type StructureData = StructureData_marker EndMarker | StructureData_tree StructureTree

type alias Structure = {
  attachments: List StructureData
}

type alias PhysicsState = {
  x: Float,
  y: Float,
  theta: Float
}

type alias Ship = {
  entityId: Int,
  structure: Structure,
  physicsState: PhysicsState
}
