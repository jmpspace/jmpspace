
{-
    Copyright (c) John P Mayer, Jr 2013

    This file is part of celestia.

    celestia is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    celestia is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with celestia.  If not, see <http://www.gnu.org/licenses/>.
-}


module Demo where

import Either (..)

import Dict (Dict)
import Dict as D

import Physics (..)

import Draw (drawEntity)
import Control.State (execState)
import Step (step)
import GameInputs (gameInputs)
import Types (..)
import Utils (..)
import Data.Vec2 (..)

import Main (simpleShip)
import ShipWright (addPhantom)

{- Setup initial game state and initialize the loop -}

spine : Structure
spine = beam {r = 200} [ ( { offset=150, theta=(pi/2) }, beam { r=40 } [] ) ] 

station : Entity
station = { controls = Right [], motion = { pos = { x = 0, y = 200, theta = 0 }, v = { x = 0, y = 0 }, omega = 0 }, cache = genEntityCache spine }

initialShips : [Entity]
initialShips = 
  [ { controls = Right [], motion = { pos = { x =   0, y =   0, theta = 0 }, v = { x = 0, y = 0 }, omega = 0 }, cache = genEntityCache simpleShip }
  , { controls = Right [], motion = { pos = { x =  60, y =  50, theta = 1 }, v = { x = 0, y = 0 }, omega = 0 }, cache = genEntityCache simpleShip }
  , { controls = Right [], motion = { pos = { x = -80, y =  40, theta = 4 }, v = { x = 0, y = 0 }, omega = 0 }, cache = genEntityCache simpleShip }
  , { controls = Right [], motion = { pos = { x = 120, y = -20, theta = 3 }, v = { x = 0, y = 0 }, omega = 0 }, cache = genEntityCache simpleShip }
  ]

initialEntities : Dict Int Entity
initialEntities = 
  D.fromList <| (4, station) :: (zip [0,1,2,3] initialShips)

initialState : GameState
initialState = { entities = initialEntities, mode = initialMode, focus = 0, cache = genGameStateCache 0 initialEntities }

initialMode : Mode
initialMode = { pause = False, build = initialBuildMode }

initialBuildMode : BuildMode
initialBuildMode = { entity = 4, stage = Place, absRotate = Nothing, placement = Nothing, part = Nothing }

current : Signal GameState
current = foldp (execState . step) initialState gameInputs

withPhantom : Signal GameState
withPhantom = addPhantom <~ current

{- Render the display -}

draw : Float -> GameState -> [Form]
draw n gs = 
  let entities = D.values <| gs.entities
      camera = gs.cache.camera
      cameraTransform = vecTranslate <| negVec camera
      entityForms = map (drawEntity <| n) entities
  in [groupTransform cameraTransform <| entityForms]

main = combineSElems outward <|
  [ spaceBlack <~ (.window <~ gameInputs) ~ (draw <~ (fst <~ timestamp gameInputs) ~ withPhantom)
  , combineSElems down
    [ (color white . asText) <~ constant "Celestia"
    , (color white . asText) <~ constant "Use WASD to control your current ship, and X to brake"
    , (color white . asText) <~ constant "Use 1-4 and the mouse to build parts on the skeleton structure"
    , (color white . asText) <~ constant "Use C to cycle between ships, including the skeleton ship"
    --, (color white . asText . prepend "Focus " . show . .focus) <~ current
    , (color white . asText . prepend "Build Part " . show . .part . .build . .mode) <~ current
    --, (color white . asText . prepend "Ship4 " . show . D.lookup 4 . .entities) <~ current
    ]
  ]
