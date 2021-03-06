package com.jmpspace.server.game.scenarios;

import com.jmpspace.contracts.SpaceServer.Physics;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass.*;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass.FloatingStructure;

import java.util.ArrayList;
import java.util.List;

public class SpawnRoom extends AbstractScenario {

  public static List<PlacedItem> roomStuff = new ArrayList<PlacedItem>() {
    {
      add(PlacedItem.newBuilder()
              .setOffsetX(10)
              .setOffsetY(10)
              .setOrientation(0)
              .setItem(Item.newBuilder().setCryoTube(CryoTube.newBuilder()))
              .build());
    }
  };

  public static Structure spawnRoom =
          Structure
                  .newBuilder()
                  .setId(0)
                  .setTree(StructureNode
                          .newBuilder()
                          .setPart(Part
                                  .newBuilder()
                                  .setPlatform(Platform
                                          .newBuilder()
                                          .setWidth(20)
                                          .setLength(20)
                                          .setPressurized(false)
                                          .addAllPlacedItems(roomStuff)
                                  ))
                                  .addAllAttachments(new ArrayList<>())

          ).build();

  public static List<FloatingStructure> spaceStuff = new ArrayList<FloatingStructure>()
  {
    {
      add(FloatingStructure.newBuilder()
              .setPhysicsState(
                      Physics.PhysicsState.newBuilder()
                      .setPosition(Physics.Vector2.newBuilder().setX(0).setY(0))
                      .setOrientation(0)
                      .setVelocity(Physics.Vector2.newBuilder().setX(0).setY(0))
                      .setSpin(0)
              )
              .setSructure(spawnRoom).build());
    }
  };

}
