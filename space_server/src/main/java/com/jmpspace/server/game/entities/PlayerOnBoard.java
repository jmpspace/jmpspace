package com.jmpspace.server.game.entities;

import com.jmpspace.server.game.ecs.Entity;
import com.jmpspace.server.game.ecs.PhysicsComponent;

public class PlayerOnBoard extends Entity implements Entity.HasPhysics {
  @Override
  public PhysicsComponent physicsComponent() {
    return null;
  }
}
