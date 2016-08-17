package com.jmpspace.server.game.entities;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.spacebase.AABB;
import co.paralleluniverse.spacebase.ElementUpdater;
import com.jmpspace.contracts.SpaceServer.Physics;
import com.jmpspace.contracts.SpaceServer.PlayerOuterClass;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass;
import com.jmpspace.server.game.Player;
import com.jmpspace.server.game.PlayerUtils;
import com.jmpspace.server.game.ecs.*;
import com.vividsolutions.jts.geom.Envelope;
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
      public WorldOuterClass.FloatingEntity.Builder calculateFloatingEntity() {
        return WorldOuterClass.FloatingEntity
                .newBuilder()
                .setPhysicsState(physicsComponent().calculatePhysicsState())
                .setEntity(WorldOuterClass.Entity
                        .newBuilder()
                        .setPlayer(PlayerOuterClass.Player
                                .newBuilder()
                        )
                );
      }
    };

  }

  @Override
  public PhysicsComponent physicsComponent() {
    return null;
  }

  @Override
  public CameraComponent cameraComponent() { return null; }

  @Override
  public GeometryComponent geometryComponent() { return null; }

  @Override
  public SerializeEntityComponent serializeEntityComponent() { return null; }


}
