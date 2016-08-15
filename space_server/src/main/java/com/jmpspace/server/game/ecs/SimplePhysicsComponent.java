package com.jmpspace.server.game.ecs;

import co.paralleluniverse.spacebase.AABB;
import co.paralleluniverse.spacebase.ElementUpdater;
import com.jmpspace.contracts.SpaceServer.Physics;
import com.jmpspace.contracts.SpaceServer.Physics.Vector2;
import com.jmpspace.server.game.physics.Utils;
import com.vividsolutions.jts.geom.Envelope;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.geom.util.AffineTransformation;

public abstract class SimplePhysicsComponent extends PhysicsComponent {

  public SimplePhysicsComponent(Entity entity, Physics.PhysicsState state, PhysicsComponent.PhysicsStepType type) {
    super(entity, state, type);
  }

  @Override
  public AABB calculateAABB() {

  }

  @Override
  void step(ElementUpdater<Entity.HasPhysics> updater) {
    Vector2 position = state.getPosition();
    Double orientation = state.getOrientation();
    Vector2 velocity = state.getVelocity();
    Double spin = state.getSpin();

    Vector2.Builder newPosition = Vector2
            .newBuilder()
            .setX(position.getX() + velocity.getX())
            .setY(position.getY() + velocity.getY());

    Double newOrientation = orientation + spin;

    state.setPosition(newPosition).setOrientation(newOrientation);

    updater.update(calculateAABB());
  }
}
