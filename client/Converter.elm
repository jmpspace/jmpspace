
module Converter where

import Control.State exposing (..)
import Json.Encode exposing (Value)

import Contracts.Common exposing (Bytes)
import Contracts.Controls exposing (..)
import Contracts.Ship exposing (..)
import Contracts.World exposing (..)
import Native.Converter
import Types exposing (part, beam)

-- TODO no good reason for this, deprecate the original Controls form Types
makeContractControls : Types.Controls -> Controls
makeContractControls c = case c of
  Types.Brakes -> Controls_brakes {}
  Types.Active groups -> Controls_active { groups = groups }

-- TODO not actually value
marshalControls : Controls -> Value
marshalControls = Native.Converter.marshalControls

-- TODO not actually value
unmarshalSnapshot : Value -> Snapshot
unmarshalSnapshot = Native.Converter.unmarshalSnapshot

type alias Node = Contracts.Ship.StructureNode
type alias Data = Contracts.Ship.StructureData

next : State (List a) (Maybe a)
next =
  get `bindS` \dataElems ->
    case dataElems of
      [] -> returnS Nothing
      (next::rem) -> put rem `bindS` \_ -> returnS (Just next)

-- TODO lots of information loss until convert the client, heh
convertPart : Part -> Types.Structure
convertPart part_contract = 
  case part_contract of
    Part_vessel vessel -> part <| Types.Brain { r = vessel.width }
    Part_fuelTank fuelTank -> part <| Types.FuelTank { l = fuelTank.length, w = fuelTank.radius }
    Part_engine engine -> part <| Types.Engine { r = engine.radius, config = engine.group }

convertAttach : Attach -> Types.Attach
convertAttach attach_contract =
  { offset = attach_contract.location, theta = attach_contract.rotation }

reconstructStructure : Structure -> Result String Types.Structure
reconstructStructure structure =
  let reconstructPart : Part -> State (List Data) (Result String Types.Structure)
      reconstructPart contract_part = 
        next `bindS` \data ->
          returnS <| 
            case data of
              Nothing -> 
                Err "No next data after part"
              Just (StructureData_tree _) -> 
                Err "No marker after part"
              Just (StructureData_marker _) -> 
                Ok <| convertPart contract_part
      
      reconstructBeam : Beam -> State (List Data) (Result String Types.Structure) 
      reconstructBeam contract_beam = 
        reconstructUntilMarker [] `bindS` \result ->
          case result of
            Err s -> returnS <| Err s
            Ok attachments ->
              returnS <| Ok <| beam { r = contract_beam.length } attachments

      reconstructUntilMarker : List (Types.Attach, Types.Structure) -> State (List Data) (Result String (List (Types.Attach, Types.Structure)))
      reconstructUntilMarker acc = 
        next `bindS` \data -> 
          case data of
            Nothing -> returnS <| Err "No next data looping until marker"
            Just (StructureData_marker _) -> returnS <| Ok acc
            Just (StructureData_tree tree) -> 
              case (tree.link, tree.node) of
                (StructureLink_root _, _) -> 
                  returnS <| Err "Root found not at root"
                (StructureLink_attach attach, node) -> 
                  reconstructNode node `bindS` \result ->
                    case result of
                      Err s -> returnS <| Err s
                      Ok structure ->
                        let acc' = (convertAttach attach, structure) :: acc
                        in reconstructUntilMarker acc'

      reconstructNode : Node -> State (List Data) (Result String Types.Structure)
      reconstructNode node = 
        case node of
          StructureNode_part part -> reconstructPart part
          StructureNode_beam beam -> reconstructBeam beam

      reconstructRoot : State (List Data) (Result String Types.Structure)
      reconstructRoot = 
        next `bindS` \data ->
          case data of
            Nothing -> returnS <| Err "No next data at root"
            Just (StructureData_marker _) -> 
              returnS <| Err "Marker at root"
            Just (StructureData_tree tree) -> 
              case (tree.link, tree.node) of
                (StructureLink_attach _, _) -> 
                  returnS <| Err "Attach found at root"
                (StructureLink_root _, node) -> 
                  reconstructNode node

  in evalState reconstructRoot structure.attachments
