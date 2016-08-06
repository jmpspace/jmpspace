package com.jmpspace.server.game;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.fibers.FiberUtil;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.spacebase.SpaceBase;
import co.paralleluniverse.spacebase.SpaceBaseBuilder;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass.World;
import com.jmpspace.server.PlayerClientActor;
import com.jmpspace.server.game.Player.GameUpdate;
import com.jmpspace.server.game.StructureActor.FloatingStructureRef;
import com.jmpspace.server.game.common.CommonRequest;
import com.jmpspace.server.game.scenarios.SpawnRoom;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.util.*;

public class Instance extends BasicActor<Instance.Request, Void> {

  public Instance(SpaceBaseWrapper spaceBaseWrapper) {
    _spaceBaseWrapper = spaceBaseWrapper;
  }

  class DirtyTable {
    boolean cryoTubes = false;
    Set<String> playersNeedingRefresh = new HashSet<>();
  }

  public static class SpaceBaseWrapper {

    static final String PLAYER_DB = "player";
    static final String LARGE_COLLIDE_DB = "large_collide";

    SpaceBase<PhysicsRef> players;
    SpaceBase<PhysicsRef> largeCollidables;

    SpaceBaseWrapper(SpaceBase<PhysicsRef> players, SpaceBase<PhysicsRef> largeCollidables) {
      this.players = players;
      this.largeCollidables = largeCollidables;
    }

    public static SpaceBaseWrapper init() {
      // FIXME: execution context, parallel or concurrent
      // TODO: put this inside of the physics manager?
      SpaceBaseBuilder builder = new SpaceBaseBuilder().setDimensions(2);
      SpaceBase<PhysicsRef> players = builder.build(PLAYER_DB);
      SpaceBase<PhysicsRef> largeCollidables = builder.build(LARGE_COLLIDE_DB);
      return new SpaceBaseWrapper(players, largeCollidables);
    }
  }

  private static final Logger logger = LogManager.getLogger(Instance.class.getName());

  private SpaceBaseWrapper _spaceBaseWrapper;

  class CryoTubeRef {
    UUID _uuid;
    ActorRef<StructureActor.Request> _structureActor;

    public CryoTubeRef(UUID uuid, ActorRef<StructureActor.Request> structureActor) {
      _uuid = uuid;
      _structureActor = structureActor;
    }
  }

  private Map<UUID, CryoTubeRef> cryoTubes = new HashMap<>();
  private Map<String, ActorRef<Player.Request>> players = new HashMap<>();
  private DirtyTable dirtyTable = new DirtyTable();


  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {

    logger.info("Starting the instance actor");

//    PhysicsManager physicsManager = new PhysicsManager();
//    ActorRef<PhysicsManager.Request> physicsManagerRef = physicsManager.spawn();

    World initialWorld = SpawnRoom.world();

    initialWorld.getFloatingStructuresList().forEach((WorldOuterClass.FloatingStructure floatingStructure) -> {

      logger.debug("Found a structure");

      FloatingStructureRef floatingStructureRef = new FloatingStructureRef(floatingStructure);
      _spaceBaseWrapper.largeCollidables.insert(floatingStructureRef, floatingStructureRef.calculateBounds());
      StructureActor structureActor = new StructureActor(floatingStructureRef, self());
      ActorRef<StructureActor.Request> structureRef = structureActor.spawn();
      floatingStructureRef._owner = structureRef;

//      _spawnPoints.put(structureRef, structureCryoTubes);

    });

    for (;;) {
      final Request message = receive();

      if (message instanceof RegisterCryoTubes) {
        RegisterCryoTubes register = (RegisterCryoTubes)message;

        @SuppressWarnings("unchecked")
        ActorRef<StructureActor.Request> structure = (ActorRef<StructureActor.Request>) register.getFrom();

        logger.info("Registering cryo tubes on {}", structure);

        register._cryoTubeIds.forEach(cryoTubeId -> {
          CryoTubeRef ref = new CryoTubeRef(cryoTubeId, structure);
          cryoTubes.put(cryoTubeId, ref);
          dirtyTable.cryoTubes = true;
        });
      }

      if (message instanceof BindToInstance) {
        BindToInstance bind = (BindToInstance)message;

        // Internal actor classes only send strongly typed objects
        @SuppressWarnings("unchecked")
        ActorRef<PlayerClientActor.Request> playerClient = (ActorRef<PlayerClientActor.Request>) bind.getFrom();
        String playerName = bind._playerName;

        logger.info("Binding player {}", playerName);

        Player player = new Player(playerName, self(), playerClient);

        // TODO: initialize player state
        ActorRef<Player.Request> ref = player.spawn();
      }

      if (message instanceof Spawn) {
        Spawn spawn = (Spawn)message;

        @SuppressWarnings("unchecked")
        ActorRef<Player.Request> player = (ActorRef<Player.Request>) spawn.getFrom();

        CryoTubeRef ref = cryoTubes.get(spawn._cryoTubeId);

        ref._structureActor.send(new StructureActor.Spawn(player, ref._uuid));
      }

      if (message instanceof GameTick) {

        // TODO: physics step!

        logger.debug("Game tick");

        // TODO: parallel?
        for (Map.Entry<String, ActorRef<Player.Request>> entry : players.entrySet()) {

          String playerName = entry.getKey();
          ActorRef<Player.Request> playerRef = entry.getValue();

          boolean refreshPlayer = dirtyTable.playersNeedingRefresh.contains(playerName);

          GameUpdate gameUpdate = new GameUpdate();
          if (refreshPlayer || dirtyTable.cryoTubes) {
            gameUpdate._cryoTubeIds = Optional.of(cryoTubes.keySet());
          }
          entry.getValue().send(gameUpdate);

        }

        dirtyTable.cryoTubes = false;
        dirtyTable.playersNeedingRefresh = new HashSet<>(); // TODO: dangerous garbage collection?

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

  public static class GameTick extends Request {}

}
