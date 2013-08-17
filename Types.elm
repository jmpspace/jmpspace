module Types where

import open State
import open TagTree
import open Vec2

{- Utils -}

cnst : a -> b -> a
cnst x = \a -> x

type Position = { x : Float, y : Float, theta : Float }

{- Structures -}

type Attach = { offset : Float, theta : Float }

translateAttach : Attach -> Vec2Ext a -> Vec2Ext a
translateAttach attach vext = 
  let dX = attach.offset * cos attach.theta
      dY = attach.offset * sin attach.theta
  in addVec { x = dX, y = dY } vext

type PointMass = { x : Float, y : Float, m : Float}

type Beam = { r : Float }

type StructureExt a = TagTree Part a Attach
type Structure = StructureExt Beam

beam : Beam -> [(Attach, Structure)] -> Structure
beam = Node

part : Part -> Structure
part = Leaf

data EngineConfig = Disabled
                  | Forward 
                  | Reverse
                  | TurnLeft 
                  | TurnRight

data Part = Brain { r : Float }
          | FuelTank { l : Float, w : Float }
          | Engine { r : Float, config : EngineConfig }

{- Labeling -}

type LabelBeamExt a = { a | id : Int }
type LabelBeam = LabelBeamExt Beam

type LabelStructure = TagTree Part LabelBeam Attach

labelStructurePart : Part -> State Int Part
labelStructurePart = returnS

fresh : State Int Int
fresh =
  bindS get (\i ->
  bindS (put (i + 1)) (\_ ->
  returnS i))

labelBeams : Structure -> LabelStructure
labelBeams s =
  let labelBeam beam = fmapS (\i -> { beam | id = i }) fresh
      modifyStructure = walkModify returnS labelBeam returnS
  in evalState (modifyStructure s) 0

{- Physics -}

type Thrust = { disp : Vec2, force : Vec2 }

type MotionState = { pos : Position, v : Vec2, omega : Float }

type MotionDelta = { a : Vec2, alpha : Float }

type Entity = { controls : [EngineConfig], motion : MotionState, structure : Structure }

{- Build -}

data BuildMode = Inactive
               | BeamMode Beam
               | PartMode Part
