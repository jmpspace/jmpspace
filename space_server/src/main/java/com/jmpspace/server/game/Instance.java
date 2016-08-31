package com.jmpspace.server.game;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.spacebase.SpaceBase;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass.World;
import com.jmpspace.server.PlayerClientActor;
import com.jmpspace.server.game.Player.GameUpdate;
import com.jmpspace.server.game.common.CommonRequest;
import com.jmpspace.server.game.ecs.Entity;
import com.jmpspace.server.game.ecs.Entity.HasPhysics;
import com.jmpspace.server.game.ecs.PhysicsComponent.PhysicsStepType;
import com.jmpspace.server.game.entities.FloatingStructure;
import com.jmpspace.server.game.physics.Queries;
import com.jmpspace.server.game.physics.Visitors;
import com.jmpspace.server.game.scenarios.SpawnRoom;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.atomic.AtomicInteger;

public class Instance extends BasicActor<Instance.Request, Void> {

  public static final double viewDistance = 50;

  private class DirtyTable {
    boolean cryoTubes = false;
    Set<String> playersNeedingRefresh = new HashSet<>();
  }

  private static final Logger logger = LogManager.getLogger(Instance.class.getName());

  private SpaceBase<HasPhysics> _spaceBase;

  public Instance(SpaceBase<HasPhysics> spaceBaseWrapper) {
    _spaceBase = spaceBaseWrapper;
  }

  private class CryoTubeRef {
    int _uuid;
    ActorRef<StructureActor.Request> _structureActor;

    CryoTubeRef(int uuid, ActorRef<StructureActor.Request> structureActor) {
      _uuid = uuid;
      _structureActor = structureActor;
    }
  }

  private Map<Integer, CryoTubeRef> cryoTubes = new HashMap<>();
  private Map<String, ActorRef<Player.Request>> players = new HashMap<>();
  private DirtyTable dirtyTable = new DirtyTable();

  private AtomicInteger stepCounter = new AtomicInteger();
  private AtomicInteger structureCounter = new AtomicInteger();

  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {

    logger.info("Starting the instance actor");

    for (WorldOuterClass.FloatingStructure floatingStructureData : SpawnRoom.spaceStuff) {

      logger.debug("Found a structure");


      // Don't use the deserialized id, generate a fresh, runtime id
      Integer structureId = structureCounter.getAndIncrement();

      FloatingStructure floatingStructure = new FloatingStructure(structureId, floatingStructureData.getSructure().getTree(), floatingStructureData.getPhysicsState());

      _spaceBase.insert(floatingStructure, floatingStructure.physicsComponent().calculateAABB());

      StructureActor structureActor = new StructureActor(floatingStructure, self(), _spaceBase);
      ActorRef<StructureActor.Request> structureRef = structureActor.spawn();

//      _spawnPoints.put(structureRef, structureCryoTubes);


    }

    for (;;) {
      final Request message = receive();

      if (message instanceof RegisterCryoTubes) {
        RegisterCryoTubes register = (RegisterCryoTubes)message;

        @SuppressWarnings("unchecked")
        ActorRef<StructureActor.Request> structure = (ActorRef<StructureActor.Request>) register.getFrom();

        logger.info("Registering cryo tubes on {}", structure);

        register.cryoTubeIds.forEach(cryoTubeId -> {
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

        if (players.containsKey(playerName)) {
          ActorRef<Player.Request> playerRef = players.get(playerName);

          logger.info("Binding player {} to existing instance {}", playerName, playerRef);

          playerRef.send(new Player.BindNewController(playerClient));

          dirtyTable.playersNeedingRefresh.add(playerName);
        } else {
          logger.info("Binding player {} to new instance", playerName);

          Player player = new Player(playerName, self(), playerClient);

          // TODO: initialize player state
          ActorRef<Player.Request> playerRef = player.spawn();

          players.put(playerName, playerRef);
          dirtyTable.playersNeedingRefresh.add(playerName);
        }
      }

      if (message instanceof PlayerNeedsRefresh) {

        PlayerNeedsRefresh needsRefresh = (PlayerNeedsRefresh)message;

        logger.debug("Marking player needing refresh {}", needsRefresh.playerName);

        dirtyTable.playersNeedingRefresh.add(needsRefresh.playerName);

      }

      if (message instanceof Spawn) {
        Spawn spawn = (Spawn)message;

        @SuppressWarnings("unchecked")
        ActorRef<Player.Request> player = (ActorRef<Player.Request>) spawn.getFrom();


        CryoTubeRef ref = cryoTubes.get(spawn._cryoTubeId);

        logger.info("Spawning player {} at {} on {}", player, spawn._cryoTubeId, ref._structureActor);

        ref._structureActor.send(new StructureActor.Spawn(player, ref._uuid));
      }

      if (message instanceof GameTick) {

        int tickStepNumber = stepCounter.getAndIncrement();

        logger.debug("Game tick");

        /*
         * This section is really the Physics System!
         */

        List<PhysicsStepType> stepPhases = new ArrayList<PhysicsStepType>() {{
          add(PhysicsStepType.Floating);
          add(PhysicsStepType.AttachedToStructure);
          add(PhysicsStepType.AttachedToPart);
        }};

        for (PhysicsStepType stepType : stepPhases) {
          logger.debug("Stepping {} entities", stepType.toString());
          _spaceBase.queryForUpdate(new Queries.AllOfPhysicsStepType(stepType), new Visitors.PhysicsStep());
        }

        _spaceBase.joinAllPendingOperations();

        ConcurrentMap<Integer, ConcurrentMap<Entity.HashSerializeEntity, Boolean>> visibleEntities = new ConcurrentHashMap<>();

        _spaceBase.join(new Queries.PlayerVisibility(), new Visitors.SaveVisibleEntities(visibleEntities));

        _spaceBase.joinAllPendingOperations();

        // TODO: parallel?

        GameUpdate gameUpdate = new GameUpdate();
        if (dirtyTable.cryoTubes) {
          gameUpdate._cryoTubeIds = Optional.of(cryoTubes.keySet());
        }
        gameUpdate.allVisibleObjects = visibleEntities;

        for (Map.Entry<String, ActorRef<Player.Request>> entry : players.entrySet()) {

          String playerName = entry.getKey();

          if (dirtyTable.playersNeedingRefresh.contains(playerName)) {
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

    int _cryoTubeId;

    public Spawn(ActorRef<?> from, int spawnId) {
      _from = from;
      _cryoTubeId = spawnId;
    }

  }

  public static class RegisterCryoTubes extends Request {

    Set<Integer> cryoTubeIds;

    public RegisterCryoTubes(ActorRef<?> from, Set<Integer> cryoTubeIds) {
      this._from = from;
      this.cryoTubeIds = cryoTubeIds;
    }

  }

  public static class GameTick extends Request {}

  public static class PlayerNeedsRefresh extends Request {

    String playerName;

    public PlayerNeedsRefresh(String playerName) {
      this.playerName = playerName;
    }
  }

}
