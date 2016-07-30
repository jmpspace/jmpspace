package com.jmpspace.server.game;

import co.paralleluniverse.actors.Actor;
import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.actors.behaviors.FromMessage;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.spacebase.AABB;
import co.paralleluniverse.spacebase.SpaceBase;
import co.paralleluniverse.spacebase.SpaceBaseBuilder;
import co.paralleluniverse.spacebase.SpatialToken;

class Manager extends BasicActor<Manager.Request, Void> {
  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {

    // FIXME: execution context, parallel or concurrent
    SpaceBaseBuilder builder = new SpaceBaseBuilder().setDimensions(2);
    SpaceBase<PhysicsRef> playerBase = builder.build("player");
//    SpaceBase ambientBase = builder.build("ambient");
//    SpaceBase largeBase = builder.build("large");
//    SpaceBase smallBase = builder.build("small");

    for (;;) {
      final Object message = receive();

      if (message instanceof Spawn) {
        Spawn spawn = (Spawn)message;

        @SuppressWarnings("unchecked")
        ActorRef<Object> playerClient = (ActorRef<Object>) spawn.getFrom();

        PhysicsRef physicsRef = new Player.FloatingRef();
        SpatialToken token = playerBase.insert(physicsRef, Player.FloatingRef.defaultBounds());
        physicsRef.set_token(token);

        Player player = new Player(new Player.Floating(physicsRef));

        // TODO: initialize player state
        player.spawn();
        playerClient.send(new Spawned(player.ref()));
      }
    }
  }

  abstract class Request implements FromMessage {}

  class Spawn extends Request {

    private ActorRef<?> _from;

    public Spawn(ActorRef<?> from) {
      _from = from;
    }

    @Override
    public ActorRef<?> getFrom() {
      return _from;
    }

  }

  abstract class Response {}

  class Spawned extends Response {

    private ActorRef<Player.Request> _spawned;

    Spawned(ActorRef<Player.Request> spawned) {
      _spawned = spawned;
    }

  }

}
