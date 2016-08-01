package com.jmpspace.server.game;

import co.paralleluniverse.actors.Actor;
import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.actors.behaviors.FromMessage;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.spacebase.AABB;
import co.paralleluniverse.spacebase.SpaceBase;
import com.jmpspace.server.PlayerClientActor;

import java.util.List;

public class Player extends BasicActor<Player.Request, Void> {

  private String _playerName;
  private ActorRef<PlayerClientActor.Request> _controller;

  abstract static class State {
//    ActorRef<Request> _owner;
  }

  public static class FloatingRef extends PhysicsRef {
    @Override
    void step(SpaceBase<PhysicsRef> base) {

    }

    static AABB defaultBounds() {
      return AABB.create(-0.5, .5, -0.5, .5);
    }
  }

  public static class Unspawned extends State {}

  public static class Floating extends State {

    PhysicsRef _ref;

    Floating(PhysicsRef ref) {
      _ref = ref;
    }

  }

  class PlatformRef {
    ActorRef<Structure.Request> actor;
    Structure structure;
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

  Player (String playerName, ActorRef<PlayerClientActor.Request> controller) {
    _playerName = playerName;
    _controller = controller;

    _state = new Unspawned();
  }

  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {

    _controller.send(new PlayerClientActor.BoundToPlayer(self()));

    for(;;) {
      Object message = receive();
    }
  }

  public abstract class Request implements FromMessage {
    @Override
    public ActorRef<?> getFrom() {
      return from;
    }

    protected ActorRef<?> from;


  }
}
