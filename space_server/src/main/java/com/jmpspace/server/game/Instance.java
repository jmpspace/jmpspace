package com.jmpspace.server.game;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.actors.behaviors.FromMessage;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.spacebase.SpaceBase;
import co.paralleluniverse.spacebase.SpaceBaseBuilder;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass.World;
import com.jmpspace.server.PlayerClientActor;
import com.jmpspace.server.game.StructureActor.FloatingStructureRef;
import com.jmpspace.server.game.scenarios.SpawnRoom;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Instance extends BasicActor<Instance.Request, Void> {

  static final String PLAYER_DB = "player";
  static final String LARGE_COLLIDE_DB = "large_collide";

  // Perhaps Structures themselves can keep track of the spawn points; and just publish a count of spawn points
  private Map<ActorRef<StructureActor.Request>, List<List<Integer>>> _spawnPoints = new HashMap<>();

  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {

    Map<String, ActorRef<Player.Request>> players = new HashMap<>();

//    PhysicsManager physicsManager = new PhysicsManager();
//    ActorRef<PhysicsManager.Request> physicsManagerRef = physicsManager.spawn();

    // FIXME: execution context, parallel or concurrent
    // TODO: put this inside of the physics manager?
    SpaceBaseBuilder builder = new SpaceBaseBuilder().setDimensions(2);
    SpaceBase<PhysicsRef> playerBase = builder.build(PLAYER_DB);
    SpaceBase<PhysicsRef> largeCollidable = builder.build(LARGE_COLLIDE_DB);

    World initialWorld = SpawnRoom.world.build();

    initialWorld.getFloatingStructuresList().forEach((WorldOuterClass.FloatingStructure floatingStructure) -> {

      FloatingStructureRef floatingStructureRef = new FloatingStructureRef(floatingStructure);
      largeCollidable.insert(floatingStructureRef, floatingStructureRef.calculateBounds());
      StructureActor structureActor = new StructureActor(floatingStructureRef, self());

      ActorRef<StructureActor.Request> structureRef = structureActor.spawn();

      List<List<Integer>> structureCryoTubes = StructureUtils.findCryoTubes(floatingStructure.getStructure());

      _spawnPoints.put(structureRef, structureCryoTubes);

    });

    for (;;) {
      final Object message = receive();

//      if ()

      if (message instanceof BindToInstance) {
        BindToInstance bind = (BindToInstance)message;

        // Internal actor classes only send strongly typed objects
        @SuppressWarnings("unchecked")
        ActorRef<PlayerClientActor.Request> playerClient = (ActorRef<PlayerClientActor.Request>) bind.getFrom();
        String playerName = bind._playerName;

//        PhysicsRef physicsRef = new Player.FloatingPlayerRef();
//        SpatialToken token = playerBase.insert(physicsRef, Player.FloatingPlayerRef.defaultBounds());
//        physicsRef.set_token(token);

        Player player = new Player(playerName, playerClient);

        // TODO: initialize player state
        ActorRef<Player.Request> ref = player.spawn();
//        playerClient.send(new Spawned(player.ref()));
      }
    }
  }

  public static abstract class Request implements FromMessage {

    protected ActorRef<?> _from;

    @Override
    public ActorRef<?> getFrom() { return _from; }

  }

  public static class BindToInstance extends Request {

    String _playerName;

    public BindToInstance(ActorRef<?> from, String playerName) {
      _from = from;
      _playerName = playerName;
    }

  }

  public class Spawn extends Request {

    public Spawn(ActorRef<?> from) {
      _from = from;
    }

  }

}
