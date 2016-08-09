package com.jmpspace.server.game;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.fibers.SuspendExecution;

public class Ticker extends BasicActor<Object, Void> {

  ActorRef<Instance.Request> _instance;

  public Ticker(ActorRef<Instance.Request> instance) {
    _instance = instance;
  }

  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {
    for (;;) {
      this.getStrand().sleep(1000);
      _instance.send(new Instance.GameTick());
    }
  }
}
