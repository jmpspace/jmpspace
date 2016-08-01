package com.jmpspace.server.game;

import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.fibers.SuspendExecution;
import com.jmpspace.server.game.common.CommonRequest;

public class PhysicsManager extends BasicActor<PhysicsManager.Request, Void> {

  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {
    return null;
  }

  public static abstract class Request extends CommonRequest {}
}
