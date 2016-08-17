package com.jmpspace.server.game.entities;

import com.jmpspace.contracts.SpaceServer.StructureOuterClass;
import com.jmpspace.server.game.StructureUtils;
import com.jmpspace.server.game.ecs.*;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.geom.util.AffineTransformation;

import java.util.concurrent.atomic.AtomicInteger;

public class Platform extends Entity implements Entity.HasPhysics, Entity.HasStaticGeometry {

  static AtomicInteger platformCounter = new AtomicInteger();

  Integer id;
  FloatingStructure floatingStructure;
  public StructureOuterClass.Platform platformPart;
  public AffineTransformation staticRelativeTransform;

  PhysicsComponent physicsComponent;
  GeometryComponent geometryComponent;

  public Platform(FloatingStructure floatingStructure, StructureOuterClass.Platform platformPart, AffineTransformation staticRelativeTransform) {
    this.id = platformCounter.getAndIncrement();
    this.floatingStructure = floatingStructure;
    this.platformPart = platformPart;
    this.staticRelativeTransform = staticRelativeTransform;

    Geometry geometry = StructureUtils.calculatePartGeometry(
            StructureOuterClass.Part.newBuilder().setPlatform(platformPart).build());

    this.geometryComponent = new GeometryComponent(this, geometry);

    this.physicsComponent = new SimpleAttachedPhysicsComponent(this, geometry, floatingStructure, PhysicsComponent.PhysicsStepType.AttachedToStructure) {
      @Override
      public AffineTransformation calculateRelativeTransform() {
        return staticRelativeTransform;
      }
    };

  }

  @Override
  public PhysicsComponent physicsComponent() { return physicsComponent; }


  @Override
  public GeometryComponent geometryComponent() { return geometryComponent; }
}
