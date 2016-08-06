package com.jmpspace.server.game;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.actors.behaviors.FromMessage;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.spacebase.AABB;
import co.paralleluniverse.spacebase.SpaceBase;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass;
import com.jmpspace.server.PlayerClientActor;
import com.jmpspace.server.game.common.CommonRequest;

import java.util.List;
import java.util.Set;
import java.util.UUID;

import static com.jmpspace.contracts.SpaceServer.WorldOuterClass.*;

public class Player extends BasicActor<Player.Request, Void> {

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
      this.physicsState = physicsState;
    }

    @Override
    AABB calculateBounds() {
      return null;
    }

    @Override
    void step(SpaceBase<PhysicsRef> base) throws InterruptedException, SuspendExecution {

    }

    static AABB defaultBounds() {
      return AABB.create(-0.5, .5, -0.5, .5);
    }

  }

  public static class Unspawned extends State {}

  public static class SpawnPending extends State {}

  public static class Floating extends State {

    PhysicsRef _ref;

    Floating(PhysicsRef ref) {
      _ref = ref;
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

      if (message instanceof SpawnCommand && _state instanceof Unspawned) {
        SpawnCommand command = (SpawnCommand)message;
        assert _controller == message.getFrom();
        _state = new SpawnPending();
        _instance.send(new Instance.Spawn(self(), command._cryoTubeId));
      }

    }
  }

  public abstract class Request extends CommonRequest {

  }

  public class SpawnCommand extends Request {
    private UUID _cryoTubeId;

    public SpawnCommand(ActorRef<PlayerClientActor.Request> from, UUID cryoTubeId) {
      _from = from;
      _cryoTubeId = cryoTubeId;
    }
  }

  public class GameUpdate extends Request {

    private Set<UUID> _cryoTubeIds;
  }

}
