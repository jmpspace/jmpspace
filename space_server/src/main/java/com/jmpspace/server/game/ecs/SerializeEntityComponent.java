package com.jmpspace.server.game.ecs;

import com.jmpspace.contracts.SpaceServer.WorldOuterClass;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass.FloatingEntity;

abstract public class SerializeEntityComponent extends ComponentBase<Void> {

  public SerializeEntityComponent(Entity entity) {
    super(entity);
  }

  abstract public FloatingEntity.Builder calculateFloatingEntity();

}
