package com.jmpspace.server.game.ecs;

import co.paralleluniverse.spacebase.ElementUpdater;
import com.jmpspace.contracts.SpaceServer.Physics;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.geom.util.AffineTransformation;

public abstract class SimpleAttachedPhysicsComponent extends PhysicsComponent {

  Geometry geometry;
  Entity.HasPhysics parentComponent;

  public SimpleAttachedPhysicsComponent(Entity entity, Geometry geometry, Entity.HasPhysics parentComponent, PhysicsStepType stepType) {
    super(entity, stepType);
    this.geometry = geometry;
    this.parentComponent = parentComponent;
  }

  @Override
  public Geometry calculateStaticGeometry() {
    return geometry;
  }

  public abstract AffineTransformation calculateRelativeTransform();

  @Override
  public AffineTransformation calculateAbsoluteTransform() {
    AffineTransformation transform = calculateRelativeTransform();
    transform.composeBefore(parentComponent.physicsComponent().calculateAbsoluteTransform());
    return transform;
  }

  @Override
  public void step(ElementUpdater<Entity.HasPhysics> updater) {
    // TODO: calculate position and orientation from this.platform
  }

  @Override
  public Physics.PhysicsState calculatePhysicsState() {
    return null;
  }
}