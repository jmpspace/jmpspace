package com.jmpspace.server.game.ecs;

import com.jmpspace.server.game.ecs.GeometryComponent;
import com.jmpspace.server.game.ecs.PhysicsComponent;

import java.util.Optional;

abstract public class Entity {

  public interface HasStructure {
    StructureComponent structureComponent();
  }

  public interface HasCamera {
    CameraComponent cameraComponent();
  }

  public interface HasStaticGeometry {
    GeometryComponent geometryComponent();
  }

  public interface HasPhysics {
    PhysicsComponent physicsComponent();
  }

  public interface HashSerializeEntity {
    SerializeEntityComponent serializeEntityComponent();
  }

}
