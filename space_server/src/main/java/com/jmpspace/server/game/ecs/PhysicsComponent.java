package com.jmpspace.server.game.ecs;

import co.paralleluniverse.spacebase.AABB;
import co.paralleluniverse.spacebase.ElementUpdater;
import com.jmpspace.contracts.SpaceServer.Physics;
import com.jmpspace.contracts.SpaceServer.Physics.PhysicsState;
import com.jmpspace.server.game.physics.Utils;
import com.vividsolutions.jts.geom.Envelope;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.geom.util.AffineTransformation;

import java.util.concurrent.atomic.AtomicInteger;

public abstract class PhysicsComponent extends ComponentBase<PhysicsComponent.PhysicsSystem> {

  static class PhysicsSystem {}
  public enum PhysicsStepType {
    Static,
    Floating,
    AttachedToStructure,
    AttachedToPart
  }
  private static AtomicInteger identityGenerator = new AtomicInteger(0);

  public int id;
  public PhysicsStepType stepType;

  protected PhysicsComponent(Entity entity, PhysicsComponent.PhysicsStepType stepType) {
    super(entity);
    this.id = identityGenerator.getAndIncrement();
    this.stepType = stepType;
  }

  public abstract Geometry calculateStaticGeometry();

  public abstract AffineTransformation calculateAbsoluteTransform();

  public AABB calculateAABB() {
    Geometry geom = (Geometry) calculateStaticGeometry().clone();
    geom.apply(calculateAbsoluteTransform());
    Envelope envelope = geom.getEnvelopeInternal();
    return AABB.create(envelope.getMinX(), envelope.getMaxX(), envelope.getMinY(), envelope.getMaxY());
  }

  public abstract void step(ElementUpdater<Entity.HasPhysics> updater);

  public abstract Physics.PhysicsState calculatePhysicsState();

}
