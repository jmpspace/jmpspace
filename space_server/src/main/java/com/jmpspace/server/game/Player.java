package com.jmpspace.server.game;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.spacebase.AABB;
import co.paralleluniverse.spacebase.SpaceBase;

import java.util.List;

public class Player extends BasicActor<Player.Request, Void> {

  abstract static class State {
    ActorRef<Request> _owner;
  }

  public static class FloatingRef extends PhysicsRef {
    @Override
    void step(SpaceBase<PhysicsRef> base) {

    }

    static AABB defaultBounds() {
      return AABB.create(-0.5, .5, -0.5, .5);
    }
  }

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

  Player (State state) {
    _state = state;
    _state._owner = self();
  }

  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {
    for(;;) {
      Object message = receive();
    }
  }

  abstract class Request {}
}
