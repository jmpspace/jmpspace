package com.jmpspace.server.game.ecs;

import com.jmpspace.contracts.SpaceServer.WorldOuterClass;

abstract public class SerializeEntityComponent extends ComponentBase<Void> {

  public SerializeEntityComponent(Entity entity) {
    super(entity);
  }

  abstract public WorldOuterClass.Entity.Builder marshalEntity();

}
