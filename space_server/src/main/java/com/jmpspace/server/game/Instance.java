package com.jmpspace.server.game;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.actors.behaviors.FromMessage;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.spacebase.SpaceBase;
import co.paralleluniverse.spacebase.SpaceBaseBuilder;
import co.paralleluniverse.spacebase.SpatialToken;
import com.jmpspace.server.PlayerClientActor;

import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ConcurrentMap;

public class Instance extends BasicActor<Instance.Request, Void> {
  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {

    Map<String, ActorRef<Player.Request>> players = new HashMap<>();

    PhysicsManager physicsManager = new PhysicsManager();
    ActorRef<PhysicsManager.Request> physicsManagerRef = physicsManager.spawn();

    // FIXME: execution context, parallel or concurrent
    // TODO: put this inside of the physics manager?
    SpaceBaseBuilder builder = new SpaceBaseBuilder().setDimensions(2);
    SpaceBase<PhysicsRef> playerBase = builder.build("player");
//    SpaceBase ambientBase = builder.build("ambient");
//    SpaceBase largeBase = builder.build("large");
//    SpaceBase smallBase = builder.build("small");

    for (;;) {
      final Object message = receive();

//      if ()

      if (message instanceof BindToInstance) {
        BindToInstance bind = (BindToInstance)message;

        // Internal actor classes only send strongly typed objects
        @SuppressWarnings("unchecked")
        ActorRef<PlayerClientActor.Request> playerClient = (ActorRef<PlayerClientActor.Request>) bind.getFrom();
        String playerName = bind._playerName;

//        PhysicsRef physicsRef = new Player.FloatingRef();
//        SpatialToken token = playerBase.insert(physicsRef, Player.FloatingRef.defaultBounds());
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
