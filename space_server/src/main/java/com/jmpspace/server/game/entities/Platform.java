package com.jmpspace.server.game.entities;

import com.jmpspace.contracts.SpaceServer.Physics;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass;
import com.jmpspace.server.game.StructureUtils;
import com.jmpspace.server.game.ecs.*;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.geom.util.AffineTransformation;

import java.util.concurrent.atomic.AtomicInteger;

public class Platform extends Entity implements Entity.HasPhysics, Entity.HasStaticGeometry, Entity.HashSerializeEntity {

  static AtomicInteger platformCounter = new AtomicInteger();

  public Integer id;
  FloatingStructure floatingStructure;
  public StructureOuterClass.Platform platformPart;
  public AffineTransformation staticRelativeTransform;
  SerializeEntityComponent serializeEntityComponent;

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

    this.serializeEntityComponent = new SerializeEntityComponent(this) {
      @Override
      public WorldOuterClass.Entity.Builder marshalEntity() {

        // TODO: faking it
        Physics.Vector2.Builder platformPosition = Physics.Vector2
                .newBuilder()
                .setX(0)
                .setY(0);

        return WorldOuterClass.Entity.newBuilder()
                .setStructurePlatform(WorldOuterClass.Platform
                        .newBuilder()
                        .setPlatform(platformPart)
                        .setStructureId(floatingStructure.id)
                        // TODO - these are fudged!
                        .setPlatformPosition(platformPosition)
                        .setPlatformRotation(0)
                );
      }
    };

  }

  @Override
  public PhysicsComponent physicsComponent() { return physicsComponent; }


  @Override
  public GeometryComponent geometryComponent() { return geometryComponent; }

  @Override
  public SerializeEntityComponent serializeEntityComponent() { return serializeEntityComponent; }
}
