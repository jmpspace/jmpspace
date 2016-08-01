package com.jmpspace.server.game.scenarios;

import com.jmpspace.contracts.SpaceServer.Game;
import com.jmpspace.contracts.SpaceServer.Game.Spawn;
import com.jmpspace.contracts.SpaceServer.Structure;
import com.jmpspace.contracts.SpaceServer.Structure.*;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass.FloatingStructure;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass.PhysicsState;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass.Vector2;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass.World;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class SpawnRoom extends AbstractScenario {

  public static List<PlacedItem.Builder> roomStuff = new ArrayList<PlacedItem.Builder>() {
    {
      PlacedItem.newBuilder()
              .setOffsetX(10)
              .setOffsetY(10)
              .setOrientation(0)
              .setItem(Item.newBuilder().setCryoTube(CryoTube.newBuilder()));
    }
  };

  public static StructureNode.Builder spawnRoom = StructureNode.newBuilder()
          .setPart(
                  Part.newBuilder()
                  .setPlatform(
                          Platform.newBuilder()
                          .setWidth(20).setLength(20).setPressurized(false)
                          .addAllPlacedItems(
                                  roomStuff.stream()
                                  .map((PlacedItem.Builder x) -> x.build())
                                  .collect(Collectors.toList())
                          )
                  ))
          .addAllAttachments(new ArrayList<>());


  public static List<FloatingStructure.Builder> spaceStuff = new ArrayList<FloatingStructure.Builder>()
  {
    {
      FloatingStructure.newBuilder()
              .setPhysicsState(
                      PhysicsState.newBuilder()
                      .setPosition(Vector2.newBuilder().setX(0).setY(0))
                      .setOrientation(0)
                      .setVelocity(Vector2.newBuilder().setX(0).setY(0))
                      .setSpin(0)
              )
              .setStructure(spawnRoom);
    }
  };

  public static World.Builder world = World.newBuilder()
          .addAllFloatingStructures(
                  spaceStuff.stream()
                          .map((FloatingStructure.Builder x) -> x.build())
                          .collect(Collectors.toList())
          );

}
