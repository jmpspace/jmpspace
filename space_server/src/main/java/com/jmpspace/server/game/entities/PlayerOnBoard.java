package com.jmpspace.server.game.entities;

import co.paralleluniverse.actors.ActorRef;
import com.jmpspace.contracts.SpaceServer.Physics;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass;
import com.jmpspace.server.game.Player;
import com.jmpspace.server.game.ecs.*;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.geom.util.AffineTransformation;
import com.vividsolutions.jts.util.GeometricShapeFactory;

import static com.jmpspace.server.game.Constants.PlayerEntityDiameter;

public class PlayerOnBoard extends Entity implements Entity.HasPhysics, Entity.HasCamera, Entity.HashSerializeEntity, Entity.HasStaticGeometry {

  Platform platform;
  Physics.Vector2 position;

  PhysicsComponent physicsComponent;
  CameraComponent cameraComponent;
  GeometryComponent geometryComponent;
  SerializeEntityComponent serializeEntityComponent;

  static AffineTransformation calculateAbsoluteTransform(Platform platform, Physics.Vector2 position) {
    AffineTransformation transform = new AffineTransformation();
    transform.translate(position.getX(), position.getY());
    platform.physicsComponent.calculateAbsoluteTransform();
    transform.composeBefore(platform.physicsComponent.calculateAbsoluteTransform());
    return transform;
//    playerGeometry.apply(transform);
//    Envelope bounds = playerGeometry.getEnvelopeInternal();
//    return AABB.create(bounds.getMinX(), bounds.getMaxX(), bounds.getMinY(), bounds.getMaxY());
  }

  public PlayerOnBoard(Platform platform, Physics.Vector2 position, ActorRef<Player.Request> owner) {

    this.platform = platform;
    this.position = position;

    // TODO: calculate position and orientation from this.platform

    GeometricShapeFactory factory = new GeometricShapeFactory();
    factory.setSize(PlayerEntityDiameter);
    Geometry staticGeometry = factory.createCircle();
    this.geometryComponent = new GeometryComponent(this, staticGeometry);

    this.physicsComponent = new SimpleAttachedPhysicsComponent(this, staticGeometry, platform, PhysicsComponent.PhysicsStepType.AttachedToPart) {
      @Override
      public AffineTransformation calculateRelativeTransform() {
        AffineTransformation transform = new AffineTransformation();
        transform.translate(position.getX(), position.getY());
        return transform;
      }
    };

    this.cameraComponent = new CameraComponent(this, owner);

    this.serializeEntityComponent = new SerializeEntityComponent(this) {
      @Override
      public WorldOuterClass.Entity.Builder marshalEntity() {

        return WorldOuterClass.Entity
                .newBuilder()
                .setOnboardPlayer(com.jmpspace.contracts.SpaceServer.Player.Onboard
                        .newBuilder()
                        .setPlatformId(platform.id)
                        .setStandingPosition(position)
                        .setStandindOrientation(0)
                );
      }
    };

  }

  @Override
  public PhysicsComponent physicsComponent() { return physicsComponent; }

  @Override
  public CameraComponent cameraComponent() { return cameraComponent; }

  @Override
  public GeometryComponent geometryComponent() { return geometryComponent; }

  @Override
  public SerializeEntityComponent serializeEntityComponent() { return serializeEntityComponent; }


}
