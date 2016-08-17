package com.jmpspace.server.game.physics;

import co.paralleluniverse.spacebase.ElementUpdater;
import co.paralleluniverse.spacebase.SpatialJoinVisitor;
import co.paralleluniverse.spacebase.SpatialModifyingVisitor;
import co.paralleluniverse.spacebase.SpatialToken;
import com.jmpspace.server.game.ecs.Entity;
import com.jmpspace.server.game.ecs.Entity.HasCamera;
import com.jmpspace.server.game.ecs.Entity.HasPhysics;
import com.jmpspace.server.game.ecs.Entity.HashSerializeEntity;
import com.jmpspace.server.game.ecs.PhysicsComponent;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class Visitors {

  public static class PhysicsStep implements SpatialModifyingVisitor<HasPhysics> {

    @Override
    public void visit(ElementUpdater<HasPhysics> elementUpdater) {
      PhysicsComponent physicsComponent = elementUpdater.elem().physicsComponent();
      physicsComponent.step(elementUpdater);
    }

    @Override
    public void done() {

    }
  }

  public static class SaveVisibleEntities implements SpatialJoinVisitor<HasPhysics, HasPhysics> {

    private ConcurrentMap<Integer, ConcurrentMap<HashSerializeEntity, Boolean>> visibleEntities;

    public SaveVisibleEntities(ConcurrentMap<Integer, ConcurrentMap<HashSerializeEntity, Boolean>> visibleEntities) {
      this.visibleEntities = visibleEntities;
    }

    @Override
    public void visit(HasPhysics playerRef, SpatialToken spatialToken, HasPhysics other, SpatialToken spatialToken1) {

      // Unchecked
      HasCamera cameraRef = (HasCamera) playerRef;
      HashSerializeEntity targetRef = (HashSerializeEntity) other;

      Integer cameraId = cameraRef.cameraComponent().id;

      visibleEntities.putIfAbsent(cameraId, new ConcurrentHashMap<>());
      visibleEntities.get(cameraId).put(targetRef, true);
    }
  }
}
