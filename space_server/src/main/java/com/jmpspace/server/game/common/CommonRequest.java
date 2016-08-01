package com.jmpspace.server.game.common;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.behaviors.FromMessage;

public class CommonRequest implements FromMessage {

  protected ActorRef<?> _from;

  @Override
  public ActorRef<?> getFrom() { return _from; }

}
