package com.jmpspace.server.game;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.spacebase.AABB;
import co.paralleluniverse.spacebase.ElementUpdater;
import com.jmpspace.contracts.SpaceServer.Game;
import com.jmpspace.contracts.SpaceServer.Game.Snapshot;
import com.jmpspace.contracts.SpaceServer.Physics.PhysicsState;
import com.jmpspace.server.PlayerClientActor;
import com.jmpspace.server.game.common.CommonRequest;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.util.*;
import java.util.concurrent.ConcurrentMap;
import java.util.stream.Collectors;

import static com.jmpspace.contracts.SpaceServer.WorldOuterClass.*;

// TODO: potential refactor, merge this and PlayerClient? not if there are multiple Instance planned...

public class Player extends BasicActor<Player.Request, Void> {

  private static final Logger logger = LogManager.getLogger(Player.class.getName());

  private String _playerName;
  private ActorRef<Instance.Request> _instance;
  private ActorRef<PlayerClientActor.Request> _controller;

  abstract static class State {
//    ActorRef<Request> _owner;
  }

  public static class FloatingPlayerRef extends PhysicsRef {

    ActorRef<Request> _owner;

    private PhysicsState physicsState;

    public FloatingPlayerRef(PhysicsState physicsState) {
      super();
      this.physicsState = physicsState;
    }

    @Override
    AABB calculateBounds() {
      return null;
    }

    @Override
    public PhysicsStepType get_physicsType() {
      return PhysicsStepType.Floating;
    }

    @Override
    public void stepPhysics(ElementUpdater<PhysicsRef> elementUpdater)  {

    }

    @Override
    void notifyOwner() throws SuspendExecution, InterruptedException {

    }

    static AABB defaultBounds() {
      return AABB.create(-0.5, .5, -0.5, .5);
    }

    @Override
    public boolean get_hasPlayerCamera() {
      return true;
    }

  }

  public static class Unspawned extends State {}

  public static class SpawnPending extends State {}

  public static class Floating extends State {

    FloatingPlayerRef _ref;

    public Floating(FloatingPlayerRef ref) {
      _ref = ref;
    }

  }

  public static class OnBoard extends State {

    StructureActor.PlayerOnBoard ref;

    public OnBoard(StructureActor.PlayerOnBoard _ref) {
      this.ref = _ref;
    }
  }

  class PlatformRef {
    ActorRef<StructureActor.Request> actor;
    StructureActor structureActor;
    List<Integer> structurePath;
  }

  public static class Walking extends State {

    PhysicsRef _ref;
    PlatformRef _platformRef;

    Walking(PhysicsRef ref, PlatformRef structureRef) {
      _ref = ref;
      _platformRef = structureRef;
    }
  }

  private State _state;

  Player (String playerName, ActorRef<Instance.Request> instance, ActorRef<PlayerClientActor.Request> controller) {
    _playerName = playerName;
    _instance = instance;
    _controller = controller;

    _state = new Unspawned();
  }

  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {

    _controller.send(new PlayerClientActor.BoundToPlayer(self()));

    for(;;) {
      Request message = (Request)receive();

      if (message instanceof GameUpdate && _state instanceof Unspawned) {

        GameUpdate gameUpdate = (GameUpdate)message;

        Snapshot.Builder builder = Snapshot.newBuilder();

        Optional<Integer> x = gameUpdate._cryoTubeIds.map(cryoTubeIds -> {
          List<String> cryoTubeStringIds = cryoTubeIds.stream().map(id -> id.toString()).collect(Collectors.toList());
          builder.setCryoTubesChange(Game.CryoTubesChange.newBuilder().addAllCryoTubeIds(cryoTubeStringIds));
          return 0;
        });

        _controller.send(new PlayerClientActor.GameSnapshot(builder.build()));

      }

      if (message instanceof GameUpdate && _state instanceof OnBoard) {

        GameUpdate gameUpdate = (GameUpdate) message;
        OnBoard onBoard = (OnBoard) _state;

        Set<PhysicsRef> myVisibleObjecs = gameUpdate.allVisibleObjects.get(onBoard.ref.getId()).keySet();

        myVisibleObjecs.stream().map(ref -> ref);

      }

      if (message instanceof GameRequest) {

        Game.GameRequest gameRequest = ((GameRequest) message).gameRequest;

        if (_state instanceof Unspawned && gameRequest.hasSpawn()) {

          Game.Spawn spawn = gameRequest.getSpawn();

          logger.debug("Spawning player {}:{} at {}", _playerName, this, spawn.getCryoTubeId());

          UUID cryoTubeId = UUID.fromString(spawn.getCryoTubeId());

          _instance.send(new Instance.Spawn(self(), cryoTubeId));

        }

      }

      if (message instanceof Spawned && _state instanceof SpawnPending) {

        Spawned spawned = (Spawned) message;

        _state = new Player.OnBoard(spawned.ref);

      }

    }
  }

  public static abstract class Request extends CommonRequest {

  }

  static class Spawned extends Request {

    StructureActor.PlayerOnBoard ref;

    public Spawned(StructureActor.PlayerOnBoard ref) {
      this.ref = ref;
    }
  }

  static class GameUpdate extends Request {

    Optional<Set<UUID>> _cryoTubeIds = Optional.empty();
    Map<Integer, ConcurrentMap<PhysicsRef, Boolean>> allVisibleObjects;



  }

  public static class GameRequest extends Request {

    Game.GameRequest gameRequest;

    public GameRequest(Game.GameRequest gameRequest) {
      this.gameRequest = gameRequest;
    }
  }

}
