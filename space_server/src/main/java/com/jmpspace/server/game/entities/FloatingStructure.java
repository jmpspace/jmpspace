package com.jmpspace.server.game.entities;

import com.jmpspace.contracts.SpaceServer.Physics;
import com.jmpspace.contracts.SpaceServer.Physics.PhysicsState;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass.StructureNode;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass;
import com.jmpspace.server.game.StructureUtils;
import com.jmpspace.server.game.ecs.*;
import com.jmpspace.server.game.ecs.Entity.HasPhysics;
import com.jmpspace.server.game.ecs.Entity.HasStaticGeometry;
import com.jmpspace.server.game.ecs.Entity.HasStructure;
import com.vividsolutions.jts.geom.Geometry;

public class FloatingStructure extends Entity implements HasStructure, HasStaticGeometry, HasPhysics, Entity.HashSerializeEntity {

  Integer id;
  StructureComponent structureComponent;
  GeometryComponent geometryComponent;
  PhysicsComponent physicsComponent;
  SerializeEntityComponent serializeEntityComponent;

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
    this.serializeEntityComponent = new SerializeEntityComponent(this) {
      @Override
      public WorldOuterClass.Entity.Builder marshalEntity() {
        return WorldOuterClass.Entity
                .newBuilder()
                .setFloatingStructure(WorldOuterClass.FloatingStructure
                        .newBuilder()
                        .setPhysicsState(state)
                        .setSructure(StructureOuterClass.Structure
                                .newBuilder()
                                .setId(id)
                                .setTree(tree))
                );
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

  @Override
  public SerializeEntityComponent serializeEntityComponent() { return serializeEntityComponent; }
}
