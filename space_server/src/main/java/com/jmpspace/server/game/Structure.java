package com.jmpspace.server.game;

import co.paralleluniverse.actors.Actor;
import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.fibers.SuspendExecution;
import com.jmpspace.contracts.SpaceServer.Structure.StructureNode;

import java.util.List;
import java.util.concurrent.ConcurrentMap;

public class Structure extends BasicActor<Structure.Request, Void> {

  class PlayerRef {
    ActorRef<Player.Request> actor;
  }

  private ConcurrentMap<List<Integer>, PlayerRef> playersOnBoard;
  private StructureNode tree;

  abstract static class State {
    ActorRef<Structure.Request> _owner;
  }

  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {
    return null;
  }

  abstract class Request {}

  class Board extends Request {
    PlayerRef player;
    List<Integer> platformPath;
  }
}
