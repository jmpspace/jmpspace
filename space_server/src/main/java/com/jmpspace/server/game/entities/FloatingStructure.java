package com.jmpspace.server.game.entities;

import com.jmpspace.contracts.SpaceServer.Physics;
import com.jmpspace.contracts.SpaceServer.Physics.PhysicsState;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass.StructureNode;
import com.jmpspace.server.game.StructureUtils;
import com.jmpspace.server.game.ecs.*;
import com.vividsolutions.jts.geom.Geometry;

public class FloatingStructure extends Entity implements Entity.HasStructure, Entity.HasStaticGeometry, Entity.HasPhysics {

  Integer id;
  StructureComponent structureComponent;
  GeometryComponent geometryComponent;
  PhysicsComponent physicsComponent;

  public FloatingStructure(Integer id, StructureNode tree, PhysicsState state) {
    this.id = id;
    this.structureComponent = new StructureComponent(this, tree);
    Geometry staticGeometry = StructureUtils.calculateStructureGeometry(tree);
    this.geometryComponent = new GeometryComponent(this, staticGeometry);
    this.physicsComponent = new SimplePhysicsComponent(this, state, PhysicsComponent.PhysicsStepType.Floating) {
      @Override
      public Geometry calculateStaticGeometry() {
        return staticGeometry;
      }
    };

  }

  @Override
  public PhysicsComponent physicsComponent() {
    return physicsComponent;
  }

  @Override
  public GeometryComponent geometryComponent() {
    return geometryComponent;
  }

  @Override
  public StructureComponent structureComponent() {
    return structureComponent;
  }
}
