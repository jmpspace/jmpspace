package com.jmpspace.server.game.ecs;

import co.paralleluniverse.spacebase.AABB;
import co.paralleluniverse.spacebase.ElementUpdater;
import com.jmpspace.contracts.SpaceServer.Physics.PhysicsState;
import com.jmpspace.server.game.PhysicsRef;
import com.jmpspace.server.game.physics.Utils;
import com.vividsolutions.jts.geom.Envelope;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.geom.util.AffineTransformation;

import java.util.concurrent.atomic.AtomicInteger;

public abstract class PhysicsComponent extends ComponentBase<PhysicsComponent.PhysicsSystem> {

  static class PhysicsSystem {}
  public enum PhysicsStepType { Static, Floating, Attached }
  private static AtomicInteger identityGenerator = new AtomicInteger(0);

  int id;
  PhysicsState.Builder state;
  PhysicsStepType type;

  PhysicsComponent(Entity entity, PhysicsState state, PhysicsComponent.PhysicsStepType type) {
    super(entity);
    this.id = identityGenerator.getAndIncrement();
    this.state = PhysicsState.newBuilder(state);
    this.type = type;
  }

  public abstract Geometry calculateStaticGeometry();

  public AffineTransformation calculateAbsoluteTransform() {
    return Utils.absoluteTransform(state);
  }

  public AABB calculateAABB() {
    Geometry geom = (Geometry) calculateStaticGeometry().clone();
    geom.apply(calculateAbsoluteTransform());
    Envelope envelope = geom.getEnvelopeInternal();
    return AABB.create(envelope.getMinX(), envelope.getMaxX(), envelope.getMinY(), envelope.getMaxY());
  }

  abstract void step(ElementUpdater<Entity.HasPhysics> updater);


}
