package com.jmpspace.server.game;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.spacebase.SpaceBase;
import co.paralleluniverse.spacebase.SpaceBaseBuilder;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass.World;
import com.jmpspace.server.PlayerClientActor;
import com.jmpspace.server.game.StructureActor.FloatingStructureRef;
import com.jmpspace.server.game.common.CommonRequest;
import com.jmpspace.server.game.scenarios.SpawnRoom;

import java.util.*;

public class Instance extends BasicActor<Instance.Request, Void> {

  static final String PLAYER_DB = "player";
  static final String LARGE_COLLIDE_DB = "large_collide";

  // Perhaps Structures themselves can keep track of the spawn points; and just publish a count of spawn points
  private Map<ActorRef<StructureActor.Request>, List<List<Integer>>> _spawnPoints = new HashMap<>();


  class CryoTubeRef {
    UUID _uuid;
    ActorRef<StructureActor.Request> structureActor;
  }

  private Map<UUID, CryoTubeRef> cryoTubes = new HashMap<>();

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
      floatingStructureRef._owner = structureRef;

//      _spawnPoints.put(structureRef, structureCryoTubes);

    });

    for (;;) {
      final Object message = receive();

      if (message instanceof BindToInstance) {
        BindToInstance bind = (BindToInstance)message;

        // Internal actor classes only send strongly typed objects
        @SuppressWarnings("unchecked")
        ActorRef<PlayerClientActor.Request> playerClient = (ActorRef<PlayerClientActor.Request>) bind.getFrom();
        String playerName = bind._playerName;

        Player player = new Player(playerName, self(), playerClient);

        // TODO: initialize player state
        ActorRef<Player.Request> ref = player.spawn();
      }

      if (message instanceof Spawn) {
        Spawn spawn = (Spawn)message;

        @SuppressWarnings("unchecked")
        ActorRef<Player.Request> player = (ActorRef<Player.Request>) spawn.getFrom();

        CryoTubeRef ref = cryoTubes.get(spawn._cryoTubeId);

        ref.structureActor.send(new StructureActor.Spawn(player, ref._uuid));
      }
    }
  }

  public static abstract class Request extends CommonRequest {}

  public static class BindToInstance extends Request {

    String _playerName;

    public BindToInstance(ActorRef<?> from, String playerName) {
      _from = from;
      _playerName = playerName;
    }

  }

  public static class Spawn extends Request {

    UUID _cryoTubeId;

    public Spawn(ActorRef<?> from, UUID spawnId) {
      _from = from;
      _cryoTubeId = spawnId;
    }

  }

  public static class RegisterCryoTubes extends Request {

    Set<UUID> _cryoTubeIds;

    public RegisterCryoTubes(ActorRef<?> from, Set<UUID> cryoTubeIds) {
      _from = from;
      _cryoTubeIds = cryoTubeIds;
    }

  }

  public static class PhysicsTick extends Request {}

}
